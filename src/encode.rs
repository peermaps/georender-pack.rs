use crate::{PeerArea, PeerLine, PeerNode, Member, MemberRole,tags};
use desert::ToBytesLE;
use failure::Error;
use osm_is_area;
use std::collections::HashMap;

pub fn node(id: u64, point: (f32, f32), tags: &[(&str, &str)]) -> Result<Vec<u8>, Error> {
    let node = PeerNode::from_tags(id, point, &tags)?;
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

pub fn way(
    id: u64,
    tags: &[(&str, &str)],
    refs: &[u64],
    deps: &HashMap<u64, (f32, f32)>,
) -> Result<Vec<u8>, Error> {
    let len = refs.len();
    if osm_is_area::way(tags, refs) {
        let (_,positions) = get_positions(&refs, &deps, false, u64::MAX)?;
        let mut area = PeerArea::from_tags(id, &tags)?;
        area.push(&positions, &vec![]);
        area.to_bytes_le()
    } else if len > 1 {
        let (_,positions) = get_positions(&refs, &deps, false, u64::MAX)?;
        let line = PeerLine::from_tags(id, &tags, &positions)?;
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
        let (_,positions) = get_positions(&refs, &deps, false, u64::MAX)?;
        let mut area = PeerArea::new(id, feature_type, labels);
        area.push(&positions, &vec![]);
        return area.to_bytes_le();
    } else if len > 1 {
        let (_,positions) = get_positions(&refs, &deps, false, u64::MAX)?;
        let line = PeerLine::new(id, feature_type, labels, &positions);
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
    mmembers = Member::sort(&mmembers, ways);

    let mut area = PeerArea::new(id, feature_type, labels);
    let mut positions = vec![];
    let mut holes = vec![];
    let mut closed = false;
    let mut ref0 = u64::MAX;

    for m in mmembers.iter() {
        match m.role {
            MemberRole::Outer() => {
                if closed {
                    area.push(&positions, &holes);
                    positions.clear();
                    holes.clear();
                    ref0 = u64::MAX;
                }
                let refs = ways.get(&m.id).unwrap();
                let (c,pts) = get_positions(refs, nodes, m.reverse, ref0)?;
                closed = c;
                positions.extend(pts);
                if closed {
                    ref0 = u64::MAX;
                } else if ref0 == u64::MAX && m.reverse {
                    ref0 = *refs.last().unwrap();
                } else if ref0 == u64::MAX {
                    ref0 = *refs.first().unwrap();
                }
            },
            MemberRole::Inner() => {
                let refs = ways.get(&m.id).unwrap();
                let (c,pts) = get_positions(refs, nodes, m.reverse, ref0)?;
                if ref0 == u64::MAX && m.reverse {
                    ref0 = *refs.last().unwrap();
                    holes.push(positions.len()/2);
                } else if ref0 == u64::MAX {
                    ref0 = *refs.first().unwrap();
                    holes.push(positions.len()/2);
                }
                if c {
                    ref0 = u64::MAX;
                }
                positions.extend(pts);
            },
            _ => {},
        }
    }
    if closed && !positions.is_empty() {
        area.push(&positions, &holes);
        positions.clear();
        holes.clear();
    }
    return area.to_bytes_le();
}

fn get_positions(
    refs: &[u64],
    nodes: &HashMap<u64, (f32, f32)>,
    reverse: bool,
    ref0: u64,
) -> Result<(bool,Vec<f32>), Error> {
    let mut positions = Vec::with_capacity(nodes.len() * 2);
    let irefs = (0..refs.len()).map(|i| refs[match reverse {
        true => refs.len()-i-1,
        false => i,
    }]);
    let mut closed = false;
    for r in irefs {
        if r == ref0 {
            closed = true;
            continue;
        }
        match nodes.get(&r) {
            Some((lon,lat)) => {
                positions.push(*lon);
                positions.push(*lat);
            },
            None => bail!("Could not find dep for {}", &r),
        }
    }
    return Ok((closed,positions));
}
