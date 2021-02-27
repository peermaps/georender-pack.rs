use crate::{PeerArea, PeerLine, PeerNode, Member};
use desert::ToBytesLE;
use failure::Error;
use osm_is_area;
use std::collections::HashMap;

pub fn node(id: u64, point: (f64, f64), tags: &Vec<(&str, &str)>) -> Result<Vec<u8>, Error> {
    let node = PeerNode::new(id, point, &tags);
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
    let bytes = way(234941233, tags, refs, &deps).unwrap();
    assert_eq!(
        "02c801b1d6837003787af941922eef41a77af941bf30ef41977af941e72fef4100",
        hex::encode(bytes)
    );
}

pub fn way(
    id: u64,
    tags: &Vec<(&str, &str)>,
    refs: &Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>,
) -> Result<Vec<u8>, Error> {
    let len = refs.len();
    if osm_is_area::way(&tags, &refs) {
        let positions = get_positions(&refs, &deps, false)?;
        let area = PeerArea::new(id, &tags, &positions);
        return area.to_bytes_le();
    } else if len > 1 {
        let positions = get_positions(&refs, &deps, false)?;
        let line = PeerLine::new(id, &tags, &positions);
        return line.to_bytes_le();
    } else {
        return Ok(vec![]);
    }
}

pub fn relation(
    id: u64,
    tags: &Vec<(&str, &str)>,
    members: &Vec<Member>,
    nodes: &HashMap<i64, (f64, f64)>,
    ways: &HashMap<i64, Vec<i64>>,
) -> Result<Vec<u8>, Error> {
    // osm_is_area only checks members.is_empty():
    if !members.is_empty() && osm_is_area::relation(&tags, &vec![0]) {
        let mut mmembers: Vec<Member> = members.to_vec();
        Member::drain(&mut mmembers, ways);
        mmembers = Member::sort(&mmembers, ways);
        let mut positions = vec![];
        for m in mmembers.iter() {
            match ways.get(&(m.id as i64)) {
                None => bail!["way member {} not given", m.id],
                Some(refs) => positions.extend(get_positions(&refs, &nodes, m.reverse)?)
            }
        }
        let area = PeerArea::new(id, &tags, &positions);
        return area.to_bytes_le();
    } else {
        return Ok(vec![]);
    }
}

fn get_positions(
    refs: &Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>,
    reverse: bool,
) -> Result<Vec<(f64, f64)>, Error> {
    let mut positions = Vec::with_capacity(deps.keys().len() * 2);
    let irefs = (0..refs.len()).map(|i| refs[match reverse {
        true => refs.len()-i-1,
        false => i,
    }]);
    for r in irefs {
        match deps.get(&r) {
            Some(point) => {
                positions.push(point.clone());
            },
            None => bail!("Could not find dep for {}", &r),
        }
    }
    return Ok(positions);
}
