use crate::{PeerArea, PeerLine, PeerNode};
use desert::ToBytesLE;
use failure::Error;
use osm_is_area;
use std::collections::HashMap;

// Some convenience functions

pub fn node(id: u64, lon: f64, lat: f64, tags: Vec<(&str, &str)>) -> Result<Vec<u8>, Error> {
    let node = PeerNode::new(id, lon, lat, &tags);
    return node.to_bytes_le();
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
