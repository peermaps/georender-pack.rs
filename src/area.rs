use crate::{parse_tags, Tags};
use desert::ToBytesLE;
use failure::Error;
use std::rc::Rc;

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

impl<'a> ToBytesLE for PeerArea<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, labels) = parse_tags(&self.tags)?;
        let pcount = self.positions.len() as u64;
        let typ_length = varinteger::length(typ);
        let id_length = varinteger::length(self.id);
        let pcount_length = varinteger::length(pcount);
        let label_length = labels.len();
        let mut buf = vec![
            0u8;
            9 + typ_length
                + id_length
                + pcount_length
                + (self.positions.len() * 8)
                + (self.positions.len() - 2) * 3 * 2  // magic copied from node version
                + label_length
        ];
        buf.push(0x02);
        let mut typbuf = vec![0u8; typ_length];
        varinteger::encode(typ, &mut typbuf);
        buf.extend(typbuf);

        let mut idbuf = vec![0u8; id_length];
        varinteger::encode(self.id, &mut idbuf);
        buf.extend(idbuf);

        let mut pcount_buf = vec![0u8; pcount_length];
        varinteger::encode(self.id, &mut pcount_buf);
        buf.extend(pcount_buf);

        // positions
        for (lon, lat) in self.positions {
            buf.extend(&lon.to_bytes_le()?);
            buf.extend(&lat.to_bytes_le()?);
        }

        buf.extend(labels);
        return Ok(buf);
    }
}
