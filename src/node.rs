use crate::varint;
use crate::{parse_tags, Tags};
use desert::ToBytesLE;
use failure::Error;
use std::rc::Rc;

/*
#[test]
fn peer_node() {
    let node = PeerNode {
        id: 14231,
    };

    let bytes = node.to_bytes_le();

}
*/

#[derive(Debug)]
pub struct PeerNode<'a> {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Rc<Tags<'a>>,
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
