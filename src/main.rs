#[macro_use] extern crate lazy_static;
mod encode;
mod osm_types;
mod schema;

use std::collections::HashMap;
use osmpbf::{ElementReader, Element};
use std::error::Error;
use std::env;

fn main() {
    run();
}
  
fn run() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();

    let reader = ElementReader::from_path(&args[1]).unwrap();

    let mut nodes: HashMap<i64, (f64, f64)> = HashMap::new();
    let mut deps: HashMap<i64, (f64, f64)> = HashMap::new();

    reader.for_each(|item| {
        match item {
            Element::DenseNode(dense) => {
               nodes.insert(dense.id, (dense.lat(), dense.lon()));
               let node = encode::dense_node(dense, &deps);
            },
            Element::Relation(_rel) => {
                // do nothing
            },
            Element::Node(node) => {
               nodes.insert(node.id(), (node.lat(), node.lon()));
               encode::node(node, &deps);
            },
            Element::Way(way) => {
                for r in way.refs() {
                   let ref item = nodes[&r];
                   deps.entry(r).or_insert(*item);
                }
               encode::way(way, &deps);
            }
        }
    }).unwrap();

    println!("refs {}", deps.len());
    println!("total nodes {}", nodes.len());

    Ok(())
}
