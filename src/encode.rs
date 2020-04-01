use osmpbf::{Node, Way, DenseNode, TagIter};
use std::collections::HashMap;
use crate::types;
use desert::{ToBytes,FromBytes};

pub struct Encoder {
  all_types: HashMap<String, i32>
}

fn is_area (way: Way) -> bool {
  let refs = way.refs();
  let len = refs.len();
  if len < 3 {
    return false;
  } else {
    let first = refs.next().unwrap();
    let last = refs.last().unwrap();
    return first == last;
  }
}

impl Encoder {
  pub fn new () -> Encoder {
    let all_types= types::get_types();
    return Encoder { all_types: all_types }
  }

  pub fn way (&self, way: Way, deps: &HashMap<i64, (f64, f64)>) -> Vec<u8> {
    let typ = self.get_type(way.tags());
    let buf;
    let refs = way.refs();
    let len = refs.len();
    if is_area(way) {
      buf = vec![0u8; 17 + len*4*2 + (len-2)*3*2];
      buf[0] = 0x03;
      buf.extend(&typ.to_be_bytes());
      buf.extend(&way.id().to_be_bytes());
      buf.extend(&(len as u16).to_be_bytes());
      for r in refs {
        // TODO: use a Point type instead of a tuple
        let lon = deps[&r].0;
        let lat = deps[&r].1;
        buf.extend(&lon.to_be_bytes());
        buf.extend(&lat.to_be_bytes());
      }
      // TODO: triangulation jazz

    } else if len > 1 {
      buf = vec![]
      buf[0] = 0x02;

    } else {
      buf = vec![];
      return buf;
    }
    return buf;
  }

  pub fn node (&self, node: Node, deps: &HashMap<i64, (f64, f64)>) -> Vec<u8> {
    let typ = self.get_type(node.tags());
    let buf: Vec<u8> = vec![0u8;21];
    buf[0] = 0x01;
    buf.extend(&typ.to_be_bytes()); 
    buf.extend(&node.id().to_be_bytes());
    buf.extend(&node.lon().to_be_bytes());
    buf.extend(&node.lat().to_be_bytes());
    return buf;
  }

  pub fn dense_node (&self, dense: DenseNode, refs: &HashMap<i64, (f64, f64)>) -> Vec<i64> {
    // parse_tags(dense.tags())
  }

  fn get_type (&self, tags: TagIter) -> i32 {
    let mut t = None;
    for tag in tags {
      let string = format!("{}.{}", tag.0, tag.1);
      if self.all_types.contains_key(&string) {
        t = self.all_types.get(&string) 
      }
    }
    match t {
      Some(_) => return *t.unwrap(),
      None => return 277 // place.other
    }
  }

}

