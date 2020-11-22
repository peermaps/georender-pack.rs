use crate::{PeerArea, PeerLine, PeerNode};
use desert::ToBytesLE;
use failure::Error;
use osm_is_area;
use std::collections::HashMap;

// Some convenience functions

pub fn node(id: u64, point: (f64, f64), tags: Vec<(&str, &str)>) -> Result<Vec<u8>, Error> {
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
    tags: Vec<(&str, &str)>,
    refs: Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>,
) -> Result<Vec<u8>, Error> {
    let len = refs.len();
    if osm_is_area::way(&tags, &refs) {
        let positions = get_positions(&refs, &deps)?;
        let area = PeerArea::new(id, &tags, &positions);
        return area.to_bytes_le();
    } else if len > 1 {
        let positions = get_positions(&refs, &deps)?;
        let line = PeerLine::new(id, &tags, &positions);
        return line.to_bytes_le();
    } else {
        return Ok(vec![]);
    }
}

fn get_positions(
    refs: &Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>,
) -> Result<Vec<(f64, f64)>, Error> {
    let mut positions = Vec::new();
    // positions
    for r in refs {
        let lon;
        let lat;
        match deps.get(r) {
            Some(dep) => {
                lon = dep.0;
                lat = dep.1;
                positions.push((lon, lat));
            }
            None => bail!("Could not find dep for {}", &r),
        }
    }
    return Ok(positions);
}
