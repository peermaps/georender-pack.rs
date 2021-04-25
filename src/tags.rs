use crate::osm_types;
use crate::tag_priorities;
use crate::varint;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;

#[test]
fn two_tags_one_has_no_priority() {
    use crate::node::PeerNode;
    use desert::ToBytesLE;
    let id = 1831881213;
    let lon = 12.253938100000001;
    let lat = 54.09006660000001;
    let tags = vec![
        ("name", "I am Stoplight"),
        ("highway", "traffic_signals"),
        ("power", "cable"),
    ];
    let node = PeerNode::from_tags(id, (lon, lat), &tags);
    let bytes = node.unwrap().to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "01d305fd93c1e906211044413a5c58420f3d4920616d2053746f706c6967687400"
    );
}

#[test]
fn two_tags_both_valid_priorities() {
    use crate::node::PeerNode;
    use desert::ToBytesLE;
    let id = 1831881213;
    let lon = 12.253938100000001;
    let lat = 54.09006660000001;
    let tags = vec![
        ("name", "I am Stoplight"),
        ("route", "canoe"),
        ("power", "cable"),
    ];
    let node = PeerNode::from_tags(id, (lon, lat), &tags);

    let bytes = node.unwrap().to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "01d305fd93c1e906211044413a5c58420f3d4920616d2053746f706c6967687400"
    );
}

#[test]
fn two_tags_same_priority() {
    use crate::node::PeerNode;
    use desert::ToBytesLE;
    let id = 1831881213;
    let lon = 12.253938100000001;
    let lat = 54.09006660000001;
    let tags = vec![
        ("name", "I am Stoplight"),
        ("railway", "wash"),
        ("power", "cable"),
    ];
    let node = PeerNode::from_tags(id, (lon, lat), &tags);

    let bytes = node.unwrap().to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "01d305fd93c1e906211044413a5c58420f3d4920616d2053746f706c6967687400"
    );

    let tags = vec![
        ("name", "I am Stoplight"),
        ("power", "cable"),
        ("railway", "wash"),
    ];
    let node = PeerNode::from_tags(id, (lon, lat), &tags);

    let bytes = node.unwrap().to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "01d305fd93c1e906211044413a5c58420f3d4920616d2053746f706c6967687400"
    );
}

pub fn get_tag(tag: &(&str, &str)) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(|[^:]+_)name($|:)").unwrap();
    }
    let pre = RE.replace(tag.0, "");
    return (pre + "=" + tag.1).to_string();
}

pub fn get_tag_length(tag: &(&str, &str)) -> usize {
    get_tag(tag).len()
}

pub fn get_label_length(tags: &[(&str, &str)]) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(|[^:]+_)name($|:)").unwrap();
    }
    let mut label_len = 1;
    for tag in tags {
        // skip all tags that aren't the name tag
        let is_name_tag = RE.find(tag.0);
        match is_name_tag {
            Some(_) => {
                let data_len = get_tag_length(tag);
                label_len += data_len + varint::length(data_len as u64);
            }
            None => {}
        }
    }
    return label_len;
}

pub fn get_tag_priority(tag: &(&str, &str)) -> Option<u64> {
    lazy_static! {
        static ref ALL_PRIORITIES: Vec<(&'static str, u64)> = tag_priorities::get_priorities();
    }

    let mut res: u64 = 0;
    for (s, p) in ALL_PRIORITIES.iter() {
        let wildcard = &format!("{}.*", tag.0);
        let formatted_key: &str = &format!("{}.{}", tag.0, tag.1);

        if *s == formatted_key || s == wildcard {
            if *p > res {
                res = *p;
            }
        }
    }
    return Some(res.clone());
}

pub fn parse(tags: &[(&str, &str)]) -> Result<(u64, Vec<u8>), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(|[^:]+_)name($|:)").unwrap();
        static ref ALL_TYPES: HashMap<&'static str, u64> = osm_types::get_types();
        static ref DEFAULT_PRIORITY: u64 = 50;
    }

    let place_other: u64 = *ALL_TYPES.get("place.other").unwrap();
    let mut label = vec![0u8; get_label_length(tags)];
    let mut top_type = place_other;
    let mut top_priority = 0;
    let mut offset = 0;

    for tag in tags {
        let formatted_key: &str = &format!("{}.{}", tag.0, tag.1);
        let priority: u64;

        match ALL_TYPES.get(formatted_key) {
            Some(this_type) => {
                //println!("Found type {} for {}", this_type, formatted_key);
                match get_tag_priority(tag) {
                    Some(_priority) => {
                        //println!("Found priority {} for {}", _priority, formatted_key);
                        priority = _priority
                    }
                    None => {
                        priority = DEFAULT_PRIORITY.clone();
                        //println!("Using default priority {} for {}", priority, formatted_key);
                    }
                }

                //println!("comparing {} top and {} priority", top_priority, priority);
                if top_priority <= priority {
                    top_priority = priority;
                    top_type = *this_type;
                    //println!("top priority {} {}", top_priority, top_type);
                }
            }
            None => {
                //println!("Found no type for {}", formatted_key);
            }
        }

        // skip all tags that aren't the name tag
        let is_name_tag = RE.find(tag.0);
        match is_name_tag {
            Some(_) => {
                let tag_length = get_tag_length(tag);

                let maybe_offset =
                    varint::encode_with_offset(tag_length as u64, &mut label, offset);
                match maybe_offset {
                    Ok(is_offset) => {
                        offset += is_offset;
                    }
                    Err(_) => {
                        bail!("Failed to encode tag {}.{}", tag.0, tag.1);
                    }
                }
                let tstr = get_tag(tag);
                tstr.bytes().for_each(|b| {
                    label[offset] = b;
                    offset += 1;
                });
            }
            None => {}
        }
    }

    label[offset] = 0x00;

    //println!("top type {}", top_type);
    return Ok((top_type, label));
}
