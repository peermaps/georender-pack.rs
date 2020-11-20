use crate::osm_types;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use varinteger;

const PLACE_OTHER: u64 = 277;

#[derive(Debug)]
pub struct Tags<'a> {
    pub iter: &'a Vec<(&'a str, &'a str)>,
}

pub fn parse_tags(tags: &Rc<Tags>) -> Result<(u64, Vec<u8>), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(|[^:]+_)name($|:)").unwrap();
        static ref ALL_TYPES: HashMap<String, u64> = osm_types::get_types();
    }

    let mut labels = vec![0u8];
    let typ;
    let mut t = None;

    for tag in tags.iter {
        let string = format!("{}.{}", tag.0, tag.1);
        if ALL_TYPES.contains_key(&string) {
            t = ALL_TYPES.get(&string);
        }

        // skip all tags that aren't the name tag
        let is_name_tag = RE.find(tag.0);
        match is_name_tag {
            Some(_) => {
                let pre = RE.replace(tag.0, "");
                let len: u64 = pre.len() as u64 + 1 + tag.1.len() as u64;
                let mut buf_len = vec![0u8; varinteger::length(len)];
                varinteger::encode(len, &mut buf_len);
                labels.extend(buf_len);
                "=".bytes().for_each(|b| labels.push(b));
                tag.1.bytes().for_each(|b| labels.push(b));
            }
            None => {}
        }
    }

    labels.push(0x00);

    match t {
        Some(_) => typ = *t.unwrap(),
        None => typ = PLACE_OTHER,
    }

    return Ok((typ, labels));
}
