use osmpbf::{Node, Way, DenseNode, TagIter};
use std::collections::HashMap;
use crate::types;

pub struct Encoder {
  all_types: HashMap<String, i32>
}

impl Encoder {
  pub fn new () -> Encoder {
    let all_types= types::get_types();
    return Encoder { all_types: all_types }
  }

  pub fn way (&self, way: Way, refs: &HashMap<i64, (f64, f64)>) {
    let typ = self.get_type(way.tags());
  }

  pub fn node (&self, node: Node, refs: &HashMap<i64, (f64, f64)>) {
    let typ = self.get_type(node.tags());
  }

  pub fn dense_node (&self, dense: DenseNode, refs: &HashMap<i64, (f64, f64)>) {
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

