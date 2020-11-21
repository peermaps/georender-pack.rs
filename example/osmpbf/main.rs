use georender::encode;
use hex;
use osmpbf::{DenseNode, Element, ElementReader, Node, Way};
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn from_node(node: Node) -> Vec<u8> {
    let tags = node.tags().into_iter().clone().collect();

    let buf = encode::node(node.id() as u64, tags, node.lon(), node.lat());
    return buf;
}

fn from_dense_node(node: DenseNode) -> Vec<u8> {
    let tags = node.tags().into_iter().collect();
    let buf = encode::node(node.id as u64, tags, node.lon(), node.lat());
    return buf;
}

fn from_way(way: Way, deps: &HashMap<i64, (f64, f64)>) -> Vec<u8> {
    let tags = way.tags().into_iter().collect();

    let refs = way.refs().into_iter().collect();
    return encode::way(way.id() as u64, tags, refs, &deps);
}

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
                Element::DenseNode(dense) => {
                    nodes.insert(dense.id, (dense.lon(), dense.lat()));
                    let encoded = from_dense_node(dense);
                    println!("{}", hex::encode(encoded));
                }
                Element::Relation(_rel) => {
                    // do nothing
                }
                Element::Node(node) => {
                    nodes.insert(node.id(), (node.lon(), node.lat()));
                    let encoded = from_node(node);
                    println!("{}", hex::encode(encoded));
                }
                Element::Way(way) => {
                    for r in way.refs() {
                        let ref item = nodes[&r];
                        deps.entry(r).or_insert(*item);
                    }
                    let encoded = from_way(way, &deps);
                    println!("{}", hex::encode(encoded));
                }
            }
        })
        .unwrap();

    Ok(())
}
