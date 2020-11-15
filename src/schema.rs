use crate::osm_types;
use desert::ToBytesLE;
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

#[derive(Debug)]
pub struct PeerLine<'a> {
    pub id: u64,
    pub positions: &'a Vec<(f32, f32)>,
    pub tags: Rc<Tags<'a>>,
}

impl<'a> PeerLine<'a> {
    pub fn new(
        id: u64,
        tags: &'a Vec<(&str, &str)>,
        positions: &'a Vec<(f32, f32)>,
    ) -> PeerLine<'a> {
        let tags = Tags { iter: tags };
        return PeerLine {
            id: id,
            positions: positions,
            tags: Rc::new(tags),
        };
    }
}

#[derive(Debug)]
pub struct PeerArea<'a> {
    pub id: u64,
    pub positions: &'a Vec<(f32, f32)>,
    pub tags: Rc<Tags<'a>>,
}

impl<'a> PeerArea<'a> {
    pub fn new(
        id: u64,
        tags: &'a Vec<(&str, &str)>,
        positions: &'a Vec<(f32, f32)>,
    ) -> PeerArea<'a> {
        let tags = Tags { iter: tags };
        return PeerArea {
            id: id,
            positions: positions,
            tags: Rc::new(tags),
        };
    }
}

#[derive(Debug)]
pub struct PeerNode<'a> {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Rc<Tags<'a>>,
}

fn parse_tags(tags: &Rc<Tags>) -> Result<(u64, Vec<u8>), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(name:|_name:)").unwrap();
        static ref ALL_TYPES: HashMap<String, u64> = osm_types::get_types();
    }

    let mut labels = vec![];
    let typ;
    let mut t = None;

    for tag in tags.iter {
        let string = format!("{}.{}", tag.0, tag.1);
        if ALL_TYPES.contains_key(&string) {
            t = ALL_TYPES.get(&string);
        }
        let parsed_key = RE.replace_all(&tag.0, ":");
        let len = parsed_key.len();
        let buf_len = &mut [0u8; 128];
        varinteger::encode(len as u64, buf_len);
        labels.extend(buf_len.into_iter());
        "=".bytes().for_each(|b| labels.push(b));
        tag.1.bytes().for_each(|b| labels.push(b));
    }

    match t {
        Some(_) => typ = *t.unwrap(),
        None => typ = PLACE_OTHER,
    }

    return Ok((typ, labels));
}

#[test]
fn peer_line() {
    let tags = vec![("source", "bing"), ("highway", "residential")];
    let positions: Vec<(f32, f32)> = vec![
        (31.184799400000003, 29.897739500000004),
        (31.184888100000002, 29.898801400000004),
        (31.184858400000003, 29.8983899),
    ];
    let id: u64 = 234941233;
    let line = PeerLine::new(id, &tags, &positions);

    let bytes = line.to_bytes_le();
    if !bytes.is_err() {
        println!("{:x}", bytes.unwrap().plain_hex(false));
    } else {
        eprintln!("{:?}", bytes.err());
    }
}

/*
#[test]
fn peer_node() {
    let node = PeerNode {
        id: 14231,
    };

    let bytes = node.to_bytes_le();
    
}
*/

impl<'a> ToBytesLE for PeerNode<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, labels) = parse_tags(&self.tags)?;
        // TODO: predict length of return value
        let mut buf = vec![0u8];
        let mut offset: usize = 0;
        buf.push(0x01);
        offset += 1;
        offset += varinteger::encode_with_offset(typ, &mut buf, offset);
        varinteger::encode_with_offset(self.id, &mut buf, offset);

        // TODO: float32 not 64
        buf.extend((self.lon as f32).to_bytes_le()?);
        buf.extend((self.lat as f32).to_bytes_le()?);
        buf.extend(labels);
        buf.push(0x00); // end labels

        return Ok(buf);
    }
}

impl<'a> ToBytesLE for PeerLine<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, labels) = parse_tags(&self.tags)?;
        let typ_length = varinteger::length(typ);
        let id_length = varinteger::length(typ);
        // TODO: predict length of return value
        let label_length = labels.len();
        let mut buf = vec![0u8; 9 + typ_length + id_length + label_length];
        println!("{}", buf.len());
        let mut offset: usize = 0;
        buf.push(0x02);
        offset += 1;
        offset += varinteger::encode_with_offset(typ, &mut buf, offset);
        offset += varinteger::encode_with_offset(self.id, &mut buf, offset);

        let len = self.positions.len();
        varinteger::encode_with_offset(len as u64, &mut buf, offset);

        // positions
        for (lon, lat) in self.positions {
            buf.extend(&lon.to_bytes_le()?);
            buf.extend(&lat.to_bytes_le()?);
        }

        buf.extend(labels);
        buf.push(0x00); // end labels
        return Ok(buf);
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
        buf.extend(&(len as u8).to_le_bytes());

        // positions
        for (lon, lat) in self.positions {
            buf.extend(&lon.to_bytes_le()?);
            buf.extend(&lat.to_bytes_le()?);
        }

        // TODO: Add Cells

        buf.extend(labels);
        buf.push(0x00); // end labels

        return Ok(buf);
    }
}
