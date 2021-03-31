use crate::osm_types;
use crate::varint;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;

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

pub fn parse(tags: &[(&str, &str)]) -> Result<(u64, Vec<u8>), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(|[^:]+_)name($|:)").unwrap();
        static ref ALL_TYPES: HashMap<&'static str, u64> = osm_types::get_types();
    }

    let place_other: u64 = *ALL_TYPES.get("place.other").unwrap();
    let mut label = vec![0u8; get_label_length(tags)];
    let typ;
    let mut t = None;
    let mut offset = 0;

    for tag in tags {
        let string: &str = &format!("{}.{}", tag.0, tag.1);
        if ALL_TYPES.contains_key(string) {
            t = ALL_TYPES.get(string);
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

    match t {
        Some(_) => typ = *t.unwrap(),
        None => typ = place_other,
    }

    return Ok((typ, label));
}
