use crate::varint;
use crate::{label, point, tags};
use desert::ToBytesLE;
use failure::Error;

#[test]
fn peer_node() {
    let id = 1831881213;
    let lon = 12.253938100000001;
    let lat = 54.09006660000001;
    let tags = vec![("name", "Neu Broderstorf"), ("traffic_sign", "city_limit")];
    let node = PeerNode::new(id, (lon, lat), &tags);

    let bytes = node.to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "019502fd93c1e906211044413a5c5842103d4e65752042726f64657273746f726600"
    );
}

#[derive(Debug)]
pub struct PeerNode<'a> {
    pub id: u64,
    pub point: (f64, f64),
    pub tags: &'a Vec<(&'a str, &'a str)>,
}

impl<'a> PeerNode<'a> {
    pub fn new(id: u64, point: (f64, f64), tags: &'a Vec<(&'a str, &'a str)>) -> PeerNode {
        return PeerNode { id, point, tags };
    }
}

impl<'a> ToBytesLE for PeerNode<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, label) = tags::parse(&self.tags)?;
        let typ_length = varint::length(typ);
        let id_length = varint::length(self.id);
        let mut buf = vec![0u8; 1 + typ_length + id_length + 2 * 4 + label.len()];
        buf[0] = 0x01;

        let mut offset = 1;
        offset += varint::encode_with_offset(typ, &mut buf, offset)?;
        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        offset += point::encode_with_offset(self.point.0, &mut buf, offset)?;
        offset += point::encode_with_offset(self.point.1, &mut buf, offset)?;

        label::encode_with_offset(&label, &mut buf, offset);
        return Ok(buf);
    }
}
