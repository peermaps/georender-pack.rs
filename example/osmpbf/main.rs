use std::collections::HashMap;
use osmpbf::{ElementReader, Element, Way, Node, DenseNode};
use std::error::Error;
use std::env;
use georender::encode;

fn from_node (node: Node)  -> Vec<u8> {
  let tags = node.tags()
    .into_iter()
    .clone()
    .collect();

  let buf = encode::node( 
      node.id(), 
      tags,
      node.lon(), 
      node.lat(),
  );
  return buf;
}

fn from_dense_node (node: DenseNode) -> Vec<u8> {
  let tags = node.tags()
    .into_iter()
    .collect();
  let buf = encode::node( 
      node.id, 
      tags,
      node.lat(), 
      node.lon()
  );
  return buf;
}

fn from_way (way: Way, deps: &HashMap<i64, (f64, f64)>) -> Vec<u8> {
  let tags = way.tags()
    .into_iter()
    .collect();

  let refs = way.refs()
    .into_iter()
    .collect();
  return encode::way(way.id(), tags, refs, &deps)
}

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
               let encoded = from_dense_node(dense);
            },
            Element::Relation(_rel) => {
                // do nothing
            },
            Element::Node(node) => {
               nodes.insert(node.id(), (node.lat(), node.lon()));
               from_node(node);
            },
            Element::Way(way) => {
                for r in way.refs() {
                   let ref item = nodes[&r];
                   deps.entry(r).or_insert(*item);
                }
               let encoded = from_way(way, &deps);
            }
        }
    }).unwrap();

    println!("refs {}", deps.len());
    println!("total nodes {}", nodes.len());

    Ok(())
}
