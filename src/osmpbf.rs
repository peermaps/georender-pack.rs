use osm_is_area;
use failure::Error;
use std::collections::HashMap;
use desert::ToBytesLE;
use crate::schema::{PeerLine, PeerArea, PeerNode};

const PLACE_OTHER: i32 = 277;
pub fn encode_way (tags: Vec<(&str, &str)>, refs: Vec<f32>) {
  let len = refs.len();
  if osm_is_area::way(&tags, &refs) {
      let area = PeerArea { 
          id: way.id(), 
          positions: get_positions(&refs, deps), 
          tags: tags 
      };
      let buf = area.to_bytes_le()?;
      return Ok(buf);
  } else if len > 1 {
    let line = PeerLine { 
        id: way.id(), 
        positions: get_positions(&refs, deps), 
        tags: tags
    };
    let buf = line.to_bytes_le()?;
    return Ok(buf);
  } else {
    return Ok(vec![])
  }
}

fn get_positions (refs: Vec<f32>, deps: &HashMap<i64, (f64, f64)>) -> Vec<(f32, f32)> {
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

