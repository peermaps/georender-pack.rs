use crate::varint;
use crate::{label, point, tags};
use desert::ToBytesLE;
use failure::Error;

#[test]
fn peer_node() -> Result<(), Error> {
    let id = 1831881213;
    let lon = 12.253938100000001;
    let lat = 54.09006660000001;
    let tags = vec![("name", "Neu Broderstorf"), ("aerialway", "cable_car")];
    let node = PeerNode::from_tags(id, (lon, lat), &tags)?;

    let bytes = node.to_bytes_le().unwrap();
    assert_eq!(
        hex::encode(bytes),
        "0100fd93c1e906211044413a5c5842103d4e65752042726f64657273746f726600"
    );
    Ok(())
}

#[derive(Debug)]
pub struct PeerNode {
    pub id: u64,
    pub point: (f32, f32),
    pub feature_type: u64,
    pub labels: Vec<u8>,
}

impl PeerNode {
    pub fn from_tags(id: u64, point: (f32, f32), tags: &[(&str, &str)]) -> Result<PeerNode, Error> {
        let (feature_type, labels) = tags::parse(tags)?;
        Ok(PeerNode {
            id,
            point,
            feature_type,
            labels,
        })
    }
    pub fn new(id: u64, point: (f32, f32), feature_type: u64, labels: &[u8]) -> PeerNode {
        PeerNode {
            id,
            point,
            feature_type,
            labels: labels.to_vec(),
        }
    }
}

impl ToBytesLE for PeerNode {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let ft_length = varint::length(self.feature_type);
        let id_length = varint::length(self.id);
        let mut buf = vec![0u8; 1 + ft_length + id_length + 2 * 4 + self.labels.len()];
        buf[0] = 0x01;

        let mut offset = 1;
        offset += varint::encode_with_offset(self.feature_type, &mut buf, offset)?;
        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        offset += point::encode_with_offset(self.point.0, &mut buf, offset)?;
        offset += point::encode_with_offset(self.point.1, &mut buf, offset)?;

        label::encode_with_offset(&self.labels, &mut buf, offset);
        Ok(buf)
    }
}
