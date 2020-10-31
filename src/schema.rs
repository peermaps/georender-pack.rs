use std::rc::Rc;
use desert::{ToBytesLE};
use regex::Regex;
use std::collections::HashMap;
use failure::Error;
use crate::osm_types;

const PLACE_OTHER: i32 = 277;

#[derive(Debug)]
pub struct Tags<'a> {
  pub iter: Vec<(&'a str, &'a str)>
}

#[derive(Debug)]
pub struct PeerLine<'a> { 
  pub id: i64,
  pub positions: Vec<(f32, f32)>,
  pub tags: Rc<Tags<'a>>
}

#[derive(Debug)]
pub struct PeerArea<'a> { 
  pub id: i64,
  pub positions: Vec<(f32, f32)>,
  pub tags: Rc<Tags<'a>>
}

#[derive(Debug)]
pub struct PeerNode<'a> { 
  pub id: i64,
  pub lat: f64,
  pub lon: f64,
  pub tags: Rc<Tags<'a>>
}

fn parse_tags (tags: &Rc<Tags>) -> Result<(i32, Vec<u8>), Error> {
  lazy_static! {
      static ref RE: Regex = Regex::new("(name:|_name:)").unwrap();
      static ref ALL_TYPES: HashMap<String, i32> = osm_types::get_types();
  }

  let mut labels = vec![];
  let typ;
  let mut t = None;

  for tag in &tags.iter {
    let string = format!("{}.{}", tag.0, tag.1);
    if ALL_TYPES.contains_key(&string) {
      t = ALL_TYPES.get(&string);
    }
    let parsed_key = RE.replace_all(&tag.0, ":");
    let len = parsed_key.len();
    labels.extend((len as u16).to_bytes_le()?);
    "=".bytes().for_each(|b| labels.push(b));
    tag.1.bytes().for_each(|b| labels.push(b));
  }

  match t {
    Some(_) => typ = *t.unwrap(),
    None => typ = PLACE_OTHER 
  }

  return Ok((typ, labels))
}

impl<'a> ToBytesLE for PeerNode<'a> {
  fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
    let (typ, labels) = parse_tags(&self.tags)?;

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

impl<'a> ToBytesLE for PeerLine<'a> {
  fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
    let len = self.positions.len();
    let (typ, labels) = parse_tags(&self.tags)?;

    // TODO: predict length of return value 
    let mut buf = vec![0u8];

    // Feature type
    buf.push(0x02);

    // Type
    buf.extend(&typ.to_bytes_le()?);

    // id
    buf.extend(&self.id.to_bytes_le()?);

    // p_count (# positions)
    buf.extend(&(len as u16).to_bytes_le()?);

    // positions
    for (lon, lat) in &self.positions {
      buf.extend(&lon.to_bytes_le()?);
      buf.extend(&lat.to_bytes_le()?);
    }

    buf.extend(labels);
    buf.push(0x00); // end labels
    return Ok(buf)
  }
}

impl<'a> ToBytesLE for PeerArea<'a> {
  fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
    let (typ, labels) = parse_tags(&self.tags)?;

    // TODO: predict length of return value 
    let mut buf = vec![0u8];

    // feature type
    buf.push(0x03);

    // type
    buf.extend(&typ.to_le_bytes());

    // id
    buf.extend(&self.id.to_le_bytes());
    
    // p_count (# of positions) 
    let len = self.positions.len();
    buf.extend(&(len as u16).to_le_bytes());

    // positions
    for (lon, lat) in &self.positions {
      buf.extend(&lon.to_bytes_le()?);
      buf.extend(&lat.to_bytes_le()?);
    }

    // TODO: Add Cells

    buf.extend(labels);
    buf.push(0x00); // end labels

    return Ok(buf)
  }
}
