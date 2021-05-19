use crate::{Area, Line, Point, Member, MemberRole, tags};
use desert::ToBytesLE;
use failure::Error;
use osm_is_area;
use std::collections::HashMap;

pub fn node(id: u64, point: (f32, f32), tags: &[(&str, &str)]) -> Result<Vec<u8>, Error> {
    let node = Point::from_tags(id, point, &tags)?;
    return node.to_bytes_le();
}

pub fn node_from_parsed(
    id: u64,
    point: (f32, f32),
    feature_type: u64,
    labels: &[u8],
) -> Result<Vec<u8>, Error> {
    let node = Point::new(id, point, feature_type, labels);
    return node.to_bytes_le();
}

#[test]
fn encode_way_line() {
    let tags = vec![("source", "bing"), ("highway", "residential")];
    let refs = vec![1, 5, 3];
    let mut deps = HashMap::new();
    deps.insert(1, (31.184799400000003, 29.897739500000004));
    deps.insert(5, (31.184888100000002, 29.898801400000004));
    deps.insert(3, (31.184858400000003, 29.8983899));
    let bytes = way(234941233, &tags, &refs, &deps).unwrap();
    assert_eq!(
        "029c03b1d6837003787af941922eef41a77af941bf30ef41977af941e72fef4100",
        hex::encode(bytes)
    );
}

#[test]
fn encode_way_area() -> Result<(),Error> {
    use crate::{decode, Feature, osm_types::get_types};
    let tags = vec![("source", "bing"), ("leisure", "park")];
    let refs = vec![1, 5, 3, 1];
    let mut deps = HashMap::new();
    deps.insert(1, (31.184799400000003, 29.897739500000004));
    deps.insert(5, (31.184888100000002, 29.898801400000004));
    deps.insert(3, (31.184858400000003, 29.8983899));
    let feature_type = *get_types().get("leisure.park").unwrap();
    let expected = Feature::Area(Area {
        id: 234941233,
        feature_type,
        labels: vec![0],
        positions: vec![
            31.184799400000003, 29.897739500000004,
            31.184888100000002, 29.898801400000004,
            31.184858400000003, 29.8983899,
        ],
        cells: vec![1,0,2],
    });
    assert_eq![&expected, &decode(&way(234941233, &tags, &refs, &deps)?)?];
    assert_eq![&expected, &decode(
        &way_from_parsed(234941233, feature_type, true, &vec![0], &refs, &deps)?
    )?];
    Ok(())
}

pub fn way(
    id: u64,
    tags: &[(&str, &str)],
    refs: &[u64],
    deps: &HashMap<u64, (f32, f32)>,
) -> Result<Vec<u8>, Error> {
    let len = refs.len();
    if osm_is_area::way(tags, refs) {
        // omit the duplicated ref for areas (first == last):
        let fixed_refs = {
            if refs.first() == refs.last() { &refs[0..refs.len()-1] }
            else { &refs }
        };
        let positions = get_way_positions(&fixed_refs, &deps)?;
        let mut area = Area::from_tags(id, &tags)?;
        area.push(&positions, &vec![]);
        area.to_bytes_le()
    } else if len > 1 {
        let positions = get_way_positions(&refs, &deps)?;
        let line = Line::from_tags(id, &tags, &positions)?;
        line.to_bytes_le()
    } else {
        Ok(vec![])
    }
}

pub fn way_from_parsed(
    id: u64,
    feature_type: u64,
    is_area: bool,
    labels: &[u8],
    refs: &[u64],
    deps: &HashMap<u64, (f32, f32)>,
) -> Result<Vec<u8>, Error> {
    let len = refs.len();
    if is_area {
        // omit the duplicated ref for areas (first == last):
        let fixed_refs = {
            if refs.first() == refs.last() { &refs[0..refs.len()-1] }
            else { &refs }
        };
        let positions = get_way_positions(&fixed_refs, &deps)?;
        let mut area = Area::new(id, feature_type, labels);
        area.push(&positions, &vec![]);
        return area.to_bytes_le();
    } else if len > 1 {
        let positions = get_way_positions(&refs, &deps)?;
        let line = Line::new(id, feature_type, labels, &positions);
        return line.to_bytes_le();
    } else {
        return Ok(vec![]);
    }
}

pub fn relation(
    id: u64,
    tags: &[(&str, &str)],
    members: &[Member],
    nodes: &HashMap<u64, (f32, f32)>,
    ways: &HashMap<u64, Vec<u64>>,
) -> Result<Vec<u8>, Error> {
    // osm_is_area only checks members.is_empty():
    let is_area = osm_is_area::relation(&tags, &vec![0]);
    let (feature_type, labels) = tags::parse(tags)?;
    relation_from_parsed(id, feature_type, is_area, &labels, members, nodes, ways)
}

pub fn relation_from_parsed(
    id: u64,
    feature_type: u64,
    is_area: bool,
    labels: &[u8],
    members: &[Member],
    nodes: &HashMap<u64, (f32, f32)>,
    ways: &HashMap<u64, Vec<u64>>,
) -> Result<Vec<u8>, Error> {
    if members.is_empty() || !is_area { return Ok(vec![]) }
    let mut mmembers: Vec<Member> = members.to_vec();
    Member::drain(&mut mmembers, ways);
    if !mmembers.iter().any(|m| m.role == MemberRole::Outer()) {
        return Ok(vec![]); // skip relations with no outers
    }
    mmembers = Member::sort(&mmembers, ways);

    let mut area = Area::new(id, feature_type, labels);
    let mut positions = vec![];
    let mut holes = vec![];
    let mut closed = false;
    let mut ref0 = u64::MAX;
    let mut prev_role = MemberRole::Unused();
    let mut prev_ref = None;
    let mut istart = 0;

    for m in mmembers.iter() {
        match m.role {
            MemberRole::Outer() => {
                if prev_role != MemberRole::Outer() {
                    if is_closed(&positions[istart..]) {
                        positions.pop();
                        positions.pop();
                    }
                    istart = positions.len();
                }
                let refs = ways.get(&m.id).unwrap();
                if closed {
                    area.push(&positions, &holes);
                    positions.clear();
                    holes.clear();
                    ref0 = u64::MAX;
                }
                let (c,pts) = get_positions(refs, nodes, m.reverse, ref0, prev_ref)?;
                prev_ref = match m.reverse {
                    true => refs.first().map(|x| x.clone()),
                    false => refs.last().map(|x| x.clone()),
                };
                closed = c;
                positions.extend(pts);
                if closed {
                    ref0 = u64::MAX;
                } else if ref0 == u64::MAX && m.reverse {
                    ref0 = *refs.first().unwrap();
                } else if ref0 == u64::MAX {
                    ref0 = *refs.last().unwrap();
                }
            }
            MemberRole::Inner() => {
                if prev_role != MemberRole::Inner() {
                    if is_closed(&positions[istart..]) {
                        positions.pop();
                        positions.pop();
                    }
                    istart = positions.len();
                }
                let refs = ways.get(&m.id).unwrap();
                let (c,pts) = get_positions(refs, nodes, m.reverse, ref0, prev_ref)?;
                prev_ref = match m.reverse {
                    true => refs.first().map(|x| x.clone()),
                    false => refs.last().map(|x| x.clone()),
                };
                if ref0 == u64::MAX && m.reverse {
                    ref0 = *refs.last().unwrap();
                    holes.push(positions.len()/2);
                } else if ref0 == u64::MAX {
                    ref0 = *refs.first().unwrap();
                    holes.push(positions.len() / 2);
                } else if c {
                    ref0 = *refs.first().unwrap();
                    holes.push(positions.len() / 2);
                }
                if c {
                    ref0 = u64::MAX;
                }
                positions.extend(pts);
            }
            _ => {}
        }
        prev_role = m.role.clone();
    }
    if !positions.is_empty() {
        if is_closed(&positions[istart..]) {
            positions.pop();
            positions.pop();
            if prev_role == MemberRole::Inner() {
                holes.push(istart/2);
            }
        }
        area.push(&positions, &holes);
        positions.clear();
        holes.clear();
    }
    return area.to_bytes_le();
}

fn get_way_positions(
    refs: &[u64],
    nodes: &HashMap<u64, (f32, f32)>,
) -> Result<Vec<f32>, Error> {
    let mut positions = Vec::with_capacity(nodes.len() * 2);
    let xrefs = if refs.first() == refs.last() {
        &refs[0..refs.len()-1]
    } else {
        &refs[..]
    };
    for r in xrefs.iter() {
        match nodes.get(&r) {
            Some((lon, lat)) => {
                positions.push(*lon);
                positions.push(*lat);
            }
            None => bail!("Could not find dep for {}", &r),
        }
    }
    return Ok(positions);
}

fn get_positions(
    refs: &[u64],
    nodes: &HashMap<u64, (f32, f32)>,
    reverse: bool,
    ref0: u64,
    prev: Option<u64>,
) -> Result<(bool,Vec<f32>), Error> {
    let mut positions = Vec::with_capacity(nodes.len() * 2);
    let fref = match reverse {
        true => *refs.last().unwrap_or(&u64::MAX),
        false => *refs.first().unwrap_or(&u64::MAX),
    };
    let irefs = (0..refs.len()).map(|i| {
        refs[match reverse {
            true => refs.len() - i - 1,
            false => i,
        }]
    });
    let mut closed = false;
    let len = irefs.len();
    for (i,r) in irefs.enumerate() {
        if i == 0 && prev == Some(r) { continue }
        if i == len-1 && prev == Some(r) { continue }
        if r == ref0 || (i > 0 && r == fref) {
            closed = true;
            continue;
        }
        match nodes.get(&r) {
            Some((lon, lat)) => {
                positions.push(*lon);
                positions.push(*lat);
            }
            None => bail!("Could not find dep for {}", &r),
        }
    }
    return Ok((closed, positions));
}

fn is_closed(pts: &[f32]) -> bool {
    if pts.len() < 4 { return false }
    let n = pts.len()-2;
    pts[0] == pts[n+0] && pts[1] == pts[n+1]
}
