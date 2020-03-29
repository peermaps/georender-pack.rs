use osmpbf::{Node, Way, DenseNode, TagIter};
use std::collections::HashMap;
use crate::types;

pub fn way (way: Way, refs: &HashMap<i64, (f64, f64)>) {
  let typ = get_type(way.tags());
  println!("{}", typ)
}

pub fn node (node: Node, refs: &HashMap<i64, (f64, f64)>) {
  let typ = get_type(node.tags());
  println!("{}", typ)

}

pub fn dense_node (dense: DenseNode, refs: &HashMap<i64, (f64, f64)>) {
  // parse_tags(dense.tags())
}

fn get_type (tags: TagIter) -> i32 {
  let all_types= types::get_types();
  let mut t = None;
  for tag in tags {
    let string = format!("{}.{}", tag.0, tag.1);
    if all_types.contains_key(&string) {
      t = all_types.get(&string) 
    }
  }
  match t {
    Some(_) => return *t.unwrap(),
    None => return 277 // place.other
  }
}