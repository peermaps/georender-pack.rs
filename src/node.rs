use crate::{parse_tags, Tags};
use desert::ToBytesLE;
use failure::Error;
use std::rc::Rc;
use varinteger;

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
        let (typ, labels) = parse_tags(&self.tags)?;
        let typ_length = varinteger::length(typ);
        let id_length = varinteger::length(self.id);
        let mut buf = vec![0u8];
        buf.push(0x01);

        let mut typbuf = vec![0u8; typ_length];
        varinteger::encode(typ, &mut typbuf);
        buf.extend(typbuf);

        let mut idbuf = vec![0u8; id_length];
        varinteger::encode(self.id, &mut idbuf);
        buf.extend(idbuf);

        buf.extend((self.lon as f32).to_bytes_le()?);
        buf.extend((self.lat as f32).to_bytes_le()?);
        buf.extend(labels);

        return Ok(buf);
    }
}
