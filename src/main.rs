mod encode;
mod types;

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
    let mut refs: HashMap<i64, (f64, f64)> = HashMap::new();

    reader.for_each(|item| {
        match item {
            Element::DenseNode(dense) => {
               nodes.insert(dense.id, (dense.lat(), dense.lon()));
               encode::dense_node(dense, &refs);
            },
            Element::Relation(_rel) => {
                // do nothing
            },
            Element::Node(node) => {
               nodes.insert( node.id(), (node.lat(), node.lon()));
               encode::node(node, &refs);
            },
            Element::Way(way) => {
                for r in way.refs() {
                   let ref item = nodes[&r];
                   refs.entry(r).or_insert(*item);
                }
               encode::way(way, &refs);
            }
        }
    }).unwrap();

    println!("refs {}", refs.len());
    println!("total nodes {}", nodes.len());

    Ok(())
}
