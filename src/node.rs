use crate::varint;
use crate::{parse_tags, Tags};
use desert::ToBytesLE;
use failure::Error;
use std::rc::Rc;

#[test]
fn peer_node() {
    let tags = vec![("name", "Neu Broderstorf"), ("traffic_sign", "city_limit")];
    let node = PeerNode::new(1831881213, 54.09006660000001, 12.253938100000001, &tags);

    let bytes = node.to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "019502fd93c1e906211044413a5c5842103d4e65752042726f64657273746f726600"
    );
}

#[derive(Debug)]
pub struct PeerNode<'a> {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Rc<Tags<'a>>,
}

impl<'a> PeerNode<'a> {
    pub fn new(id: u64, lat: f64, lon: f64, tags: &'a Vec<(&str, &str)>) -> PeerNode<'a> {
        let tags = Tags { iter: tags };
        return PeerNode {
            id: id,
            lat: lat,
            lon: lon,
            tags: Rc::new(tags),
        };
    }
}

impl<'a> ToBytesLE for PeerNode<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, label) = parse_tags(&self.tags)?;
        let typ_length = varint::length(typ);
        let id_length = varint::length(self.id);
        let mut buf = vec![0u8; 1 + typ_length + id_length];
        buf[0] = 0x01;

        let mut offset = 1;
        offset += varint::encode_with_offset(typ, &mut buf, offset)?;
        varint::encode_with_offset(self.id, &mut buf, offset)?;

        buf.extend((self.lon as f32).to_bytes_le()?);
        buf.extend((self.lat as f32).to_bytes_le()?);
        buf.extend(label);

        return Ok(buf);
    }
}
