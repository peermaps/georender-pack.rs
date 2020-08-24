use osmpbf::{Node, Way, DenseNode};
use failure::Error;
use std::collections::HashMap;
use desert::ToBytesLE;
use crate::schema::{PeerLine, PeerArea, PeerNode, Tag};

const PLACE_OTHER: i32 = 277;

fn is_area (refs: &[i64]) -> bool {
  let len = refs.len();
  if len < 3 {
    return false;
  } else {
    let first = refs[0];
    let last = refs[len - 1];
    return first == last;
  }
}

fn get_positions (refs: &[i64], deps: &HashMap<i64, (f64, f64)>) -> Vec<(f32, f32)> {
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
      },
      None => println!("Could not find dep for {}", &r)
    }
  }
  return positions;
}

pub fn way (way: Way, deps: &HashMap<i64, (f64, f64)>) -> Result<Vec<u8>, Error> {
  let tags = way.tags()
    .into_iter()
    .map(|a| Tag { K: String::from(a.0), V: String::from(a.1) })
    .collect();

  let refs = way.raw_refs();
  let len = refs.len();
  if is_area(refs) {
    let area = PeerArea { id: way.id(), positions: get_positions(refs, deps), tags };
    let buf = area.to_bytes_le()?;
    return Ok(buf);
  } else if len > 1 {
    let line = PeerLine { id: way.id(), positions: get_positions(refs, deps), tags };
    let buf = line.to_bytes_le()?;
    return Ok(buf);
  } else {
    return Ok(vec![])
  }
}

pub fn node (node: Node) -> Vec<u8> {
  // TODO: reuse code in dense_node
  let mut bytes: Vec<u8> = vec![];
  bytes[0] = 0x01;
  return bytes;
}

pub fn dense_node (node: DenseNode) -> Result<Vec<u8>, Error> {
  let tags = node.tags()
    .into_iter()
    .map(|a| Tag { K: String::from(a.0), V: String::from(a.1) })
    .collect();
  let node = PeerNode { id: node.id, lat: node.lat(), lon: node.lon(), tags };
  return node.to_bytes_le();
}