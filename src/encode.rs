use osmpbf::{Node, Way, DenseNode, WayRefIter};
use failure::Error;
use std::collections::HashMap;
use desert::ToBytesLE;
use crate::schema::{PeerLine, PeerArea, PeerNode, Tag};

const PLACE_OTHER: i32 = 277;

fn get_positions (refs: WayRefIter, deps: &HashMap<i64, (f64, f64)>) -> Vec<(f32, f32)> {
  let mut positions = Vec::new();
  // positions
  for r in refs {
    let lat;
    let lon;
    match deps.get(&r) {
      Some(dep) => {
        lon = dep.0;
        lat = dep.1;
        positions.push((lon as f32, lat as f32));
      },
      None => println!("Could not find dep for {}", &r)
    }
  }
  return positions;
}

pub fn way (way: Way, deps: &HashMap<i64, (f64, f64)>) -> Result<Vec<u8>, Error> {
  let tags = way.tags()
    .into_iter()
    .map(|a| Tag { key: String::from(a.0), value: String::from(a.1) })
    .collect();

  let mut refs = way.refs();
  let len = refs.len();
  if len >= 3 {
    let first = refs.next();
    let last = refs.last();
    if first == last {
      let area = PeerArea { id: way.id(), positions: get_positions(way.refs(), deps), tags };
      let buf = area.to_bytes_le()?;
      return Ok(buf);
    }
  }

  if len > 1 {
    let line = PeerLine { id: way.id(), positions: get_positions(way.refs(), deps), tags };
    let buf = line.to_bytes_le()?;
    return Ok(buf);
  } else {
    return Ok(vec![])
  }
}

pub fn node (node: Node)  -> Result<Vec<u8>, Error> {
  let tags = node.tags()
    .into_iter()
    .map(|a| Tag { key: String::from(a.0), value: String::from(a.1) })
    .collect();
  let node = PeerNode { id: node.id(), lat: node.lat(), lon: node.lon(), tags };
  return node.to_bytes_le();
}

pub fn dense_node (node: DenseNode) -> Result<Vec<u8>, Error> {
  let tags = node.tags()
    .into_iter()
    .map(|a| Tag { key: String::from(a.0), value: String::from(a.1) })
    .collect();
  let node = PeerNode { id: node.id, lat: node.lat(), lon: node.lon(), tags };
  return node.to_bytes_le();
}
