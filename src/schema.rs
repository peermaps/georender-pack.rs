use desert::{ToBytesLE,FromBytesLE};
use regex::Regex;
use std::collections::HashMap;
use osmpbf::{DenseTagIter};
use failure::Error;
use crate::osm_types;

const PLACE_OTHER: i32 = 277;

const all_types: HashMap<String, i32> = osm_types::get_types();

#[derive(Debug)]
pub struct Tag { 
  K: String,
  V: String
}

#[derive(Debug)]
pub struct PeerArea { 
  id: i64,
  refs: Vec<i64>,
  deps: HashMap<i64, (f64, f64)>,
  tags: Vec<Tag>
}

#[derive(Debug)]
pub struct PeerNode { 
  id: i64,
  lat: f64,
  lon: f64,
  tags: Vec<Tag>
}

fn parse_tags (tags: Vec<Tag>) -> Result<(i32, Vec<u8>), Error> {
  lazy_static! {
      static ref RE: Regex = Regex::new("(name|name_").unwrap();
  }

  let labels = vec![];
  let typ;
  let mut t = None;

  for tag in tags {
    // TODO: there must be a better way?
    let string = format!("{}.{}", tag.K, tag.V);
    if all_types.contains_key(&string) {
      t = all_types.get(&string);
    }
    let parsed_key = RE.replace_all(&tag.K, "");
    let len = parsed_key.len();
    labels.extend((len as u16).to_bytes_le()?);
    "=".bytes().map(|b| labels.push(b));
    tag.V.bytes().map(|b| labels.push(b));
  }

  match t {
    Some(_) => typ = *t.unwrap(),
    None => typ = PLACE_OTHER 
  }

  return Ok((typ, labels))
}

impl ToBytesLE for PeerNode {
  fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
    let (typ, labels) = parse_tags(self.tags)?;

    // TODO: predict length of return value 
    let mut buf = vec![0u8];
    buf.push(0x01);
    buf.extend(typ.to_bytes_le()?);
    buf.extend(self.id.to_bytes_le()?);

    // TODO: float32 not 64
    buf.extend((self.lon as f32).to_bytes_le()?);
    buf.extend((self.lat as f32).to_bytes_le()?);
    buf.extend(labels);
    buf.push(0x00); // end labels

    return Ok(buf)
  }
}

impl ToBytesLE for PeerArea {
  fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
    let len = self.refs.len();
    let (typ, labels) = parse_tags(self.tags)?;

    // TODO: predict length of return value 
    let mut buf = vec![0u8];
    buf.push(0x03);

    buf.extend(&typ.to_le_bytes());
    buf.extend(&self.id.to_le_bytes());
    buf.extend(&(len as u16).to_le_bytes());

    for r in self.refs {
      let lat;
      let lon;
      match self.deps.get(&r) {
        Some(dep) => {
          lon = dep.0;
          lat = dep.1;
          buf.extend(&(lon as f32).to_le_bytes());
          buf.extend(&(lat as f32).to_le_bytes());
        },
        None => println!("Could not find dep for {}", &r)
      }
    }
    return Ok(buf)
  }
}