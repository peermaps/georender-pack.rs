use georender::encode;
use hex;
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() {
    run();
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let reader = ElementReader::from_path(&args[1]).unwrap();

    let mut nodes: HashMap<i64, (f64, f64)> = HashMap::new();
    let mut deps: HashMap<i64, (f64, f64)> = HashMap::new();

    reader
        .for_each(|item| {
            match item {
                Element::DenseNode(node) => {
                    nodes.insert(node.id, (node.lon(), node.lat()));
                    let tags = node.tags().into_iter().collect();
                    let encoded =
                        encode::node(node.id as u64, node.lon(), node.lat(), tags).unwrap();
                    println!("{}", hex::encode(encoded));
                }
                Element::Relation(_rel) => {
                    // do nothing
                }
                Element::Node(node) => {
                    nodes.insert(node.id(), (node.lon(), node.lat()));
                    let tags = node.tags().into_iter().collect();
                    let encoded =
                        encode::node(node.id() as u64, node.lon(), node.lat(), tags).unwrap();
                    println!("{}", hex::encode(encoded));
                }
                Element::Way(way) => {
                    for r in way.refs() {
                        let ref item = nodes[&r];
                        deps.entry(r).or_insert(*item);
                    }
                    let tags = way.tags().into_iter().collect();
                    let refs = way.refs().into_iter().collect();
                    let encoded = encode::way(way.id() as u64, tags, refs, &deps).unwrap();
                    println!("{}", hex::encode(encoded));
                }
            }
        })
        .unwrap();

    Ok(())
}
