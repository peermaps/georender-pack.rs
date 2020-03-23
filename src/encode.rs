use crate::types;
use std::fs::File;
use std::io::Read;
use serde_json;
use osmpbf::{Node, Way, DenseNode, TagIter};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Basic {}

fn all_types () {
    let path = "./features.json";
    let mut file = File::open("text.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Basic = serde_json::from_str(&data).unwrap();
}

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

fn get_type (tags: TagIter) -> usize {
  let t: Option<&usize>;
  for tag in tags {
    t = types::parse(&format!("{}.{}", tag.0, tag.1));
  }

  match t {
    Some(_) => return *t.unwrap(),
    None => return 277 // place.other
  }
}