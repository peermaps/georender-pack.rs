use crate::schema::{PeerArea, PeerLine, PeerNode, Tags};
use desert::ToBytesLE;
use osm_is_area;
use std::collections::HashMap;
use std::rc::Rc;

pub fn node(id: u64, tags: Vec<(&str, &str)>, lat: f64, lon: f64) -> Vec<u8> {
    let node = PeerNode {
        id: id,
        tags: Rc::new(Tags { iter: &tags }),
        lat: lat,
        lon: lon,
    };
    return node.to_bytes_le().unwrap();
}

pub fn way(
    id: u64,
    tags: Vec<(&str, &str)>,
    refs: Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>,
) -> Vec<u8> {
    let len = refs.len();
    if osm_is_area::way(&tags, &refs) {
        let positions = get_positions(&refs, &deps);
        let area = PeerArea::new(id, &tags, &positions);
        let buf = area.to_bytes_le().unwrap();
        return buf;
    } else if len > 1 {
        let positions = get_positions(&refs, &deps);
        let line  = PeerLine::new(id, &tags, &positions);
        let buf = line.to_bytes_le().unwrap();
        return buf;
    } else {
        return vec![];
    }
}

fn get_positions(refs: &Vec<i64>, deps: &HashMap<i64, (f64, f64)>) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    // positions
    for r in refs {
        let lat;
        let lon;
        match deps.get(r) {
            Some(dep) => {
                lon = dep.0;
                lat = dep.1;
                positions.push((lon as f32, lat as f32));
            }
            None => println!("Could not find dep for {}", &r),
        }
    }
    return positions;
}
