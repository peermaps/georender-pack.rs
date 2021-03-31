use georender_pack::{encode,Member,MemberType,MemberRole};
use hex;
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let reader = ElementReader::from_path(&args[1]).unwrap();

    let mut nodes: HashMap<u64, (f32, f32)> = HashMap::new();
    let mut ways: HashMap<u64, Vec<u64>> = HashMap::new();

    reader.for_each(|item| {
        match item {
            Element::DenseNode(node) => {
                let point = (node.lon() as f32, node.lat() as f32);
                nodes.insert(node.id as u64, point);
                let tags = node.tags().into_iter().collect::<Vec<_>>();
                let encoded = encode::node(node.id as u64, point, &tags).unwrap();
                println!("{}", hex::encode(encoded));
            },
            Element::Node(node) => {
                let point = (node.lon() as f32, node.lat() as f32);
                nodes.insert(node.id() as u64, point);
                let tags = node.tags().into_iter().collect::<Vec<_>>();
                let encoded = encode::node(node.id() as u64, point, &tags).unwrap();
                println!("{}", hex::encode(encoded));
            },
            Element::Relation(rel) => {
                let tags = rel.tags().into_iter().collect::<Vec<_>>();
                let members = rel.members().map(|m| convert_member(&m)).collect::<Vec<_>>();
                let encoded = encode::relation(rel.id() as u64, &tags, &members, &nodes, &ways).unwrap();
                println!("{}", hex::encode(encoded));
            },
            Element::Way(way) => {
                let tags = way.tags().into_iter().collect::<Vec<_>>();
                let refs = way.refs().map(|r| r as u64).collect::<Vec<u64>>();
                ways.insert(way.id() as u64, refs.clone());
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
