use crate::varint;
use crate::{parse_tags, point};
use desert::ToBytesLE;
use failure::Error;

#[test]
fn peer_node() {
    let tags = vec![("name", "Neu Broderstorf"), ("traffic_sign", "city_limit")];
    let node = PeerNode::new(1831881213, 12.253938100000001, 54.09006660000001, &tags);

    let bytes = node.to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "019502fd93c1e906211044413a5c5842103d4e65752042726f64657273746f726600"
    );
}

#[derive(Debug)]
pub struct PeerNode {
    pub id: u64,
    pub lon: f64,
    pub lat: f64,
    pub typ: u64,
    pub label: Vec<u8>,
}

impl PeerNode {
    pub fn new(id: u64, lon: f64, lat: f64, tags: &Vec<(&str, &str)>) -> PeerNode {
        let (typ, label) = parse_tags(&tags);
        return PeerNode {
            id: id,
            lon: lon,
            lat: lat,
            typ: typ,
            label: label,
        };
    }
}

impl ToBytesLE for PeerNode {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let typ_length = varint::length(self.typ);
        let id_length = varint::length(self.id);
        let mut buf = vec![0u8; 1 + typ_length + id_length + 2 * 4];
        buf[0] = 0x01;

        let mut offset = 1;
        offset += varint::encode_with_offset(self.typ, &mut buf, offset)?;
        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        offset += point::encode_with_offset(self.lon, &mut buf, offset)?;
        offset += point::encode_with_offset(self.lat, &mut buf, offset)?;
        buf.extend(&self.label);

        return Ok(buf);
    }
}
