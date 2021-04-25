use crate::varint;
use crate::{label, tags};
use desert::{ToBytesLE,FromBytesLE};
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
        hex::encode(&bytes),
        "0100fd93c1e906211044413a5c5842103d4e65752042726f64657273746f726600"
    );
    assert_eq!(
        PeerNode::from_bytes_le(&bytes)?,
        (bytes.len(),node)
    );
    Ok(())
}

#[derive(Debug,Clone,PartialEq)]
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

        offset += self.point.0.write_bytes_le(&mut buf[offset..])?;
        offset += self.point.1.write_bytes_le(&mut buf[offset..])?;
        label::encode_with_offset(&self.labels, &mut buf, offset);
        Ok(buf)
    }
}

impl FromBytesLE for PeerNode {
    fn from_bytes_le(buf: &[u8]) -> Result<(usize,Self), Error> {
        if buf[0] != 0x01 {
            failure::bail!["parsing node failed. expected 0x01, received 0x{:02x}", buf[0]];
        }
        let mut offset = 1;
        let (s,feature_type) = varint::decode(&buf[offset..])?;
        offset += s;
        let (s,id) = varint::decode(&buf[offset..])?;
        offset += s;
        let (s,lon) = f32::from_bytes_le(&buf[offset..])?;
        offset += s;
        let (s,lat) = f32::from_bytes_le(&buf[offset..])?;
        offset += s;
        let s = label::scan(&buf[offset..])?;
        let labels = buf[offset..offset+s].to_vec();
        offset += s;
        Ok((offset, Self { id, point: (lon,lat), feature_type, labels }))
    }
}
