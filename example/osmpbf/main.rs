use georender_pack::{encode,Member,MemberType,MemberRole};
use hex;
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let reader = ElementReader::from_path(&args[1]).unwrap();

    let mut nodes: HashMap<i64, (f64, f64)> = HashMap::new();
    let mut ways: HashMap<i64, Vec<i64>> = HashMap::new();

    reader.for_each(|item| {
        match item {
            Element::DenseNode(node) => {
                let point = (node.lon(), node.lat());
                nodes.insert(node.id, point);
                let tags = node.tags().into_iter().collect();
                let encoded = encode::node(node.id as u64, point, &tags).unwrap();
                println!("{}", hex::encode(encoded));
            },
            Element::Node(node) => {
                let point = (node.lon(), node.lat());
                nodes.insert(node.id(), point);
                let tags = node.tags().into_iter().collect();
                let encoded = encode::node(node.id() as u64, point, &tags).unwrap();
                println!("{}", hex::encode(encoded));
            },
            Element::Relation(rel) => {
                let tags = rel.tags().into_iter().collect();
                let members = rel.members().map(|m| convert_member(&m)).collect();
                let encoded = encode::relation(rel.id() as u64, &tags, &members, &nodes, &ways).unwrap();
                println!("{}", hex::encode(encoded));
            },
            Element::Way(way) => {
                let tags = way.tags().into_iter().collect();
                let refs: Vec<i64> = way.refs().into_iter().collect();
                ways.insert(way.id(), refs.clone());
                let encoded = encode::way(way.id() as u64, &tags, &refs, &nodes).unwrap();
                println!("{}", hex::encode(encoded));
            }
        }
    }).unwrap();
    Ok(())
}

fn convert_member(m: &osmpbf::RelMember) -> Member {
    Member::new(
        m.member_id as u64,
        match m.role().unwrap() {
            "inner" => MemberRole::Inner(),
            "outer" => MemberRole::Outer(),
            _ => MemberRole::Unused(),
        },
        match m.member_type {
            osmpbf::RelMemberType::Node => MemberType::Node(),
            osmpbf::RelMemberType::Way => MemberType::Way(),
            osmpbf::RelMemberType::Relation => MemberType::Relation(),
        }
    )
}
