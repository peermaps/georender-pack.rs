use crate::varint;
use crate::{parse_tags, Tags};
use desert::ToBytesLE;
use failure::Error;
use hex;
use std::rc::Rc;

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

    let bytes = line.to_bytes_le().unwrap();
    println!("{}", hex::encode(bytes));
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

impl<'a> ToBytesLE for PeerLine<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, labels) = parse_tags(&self.tags)?;
        let pcount = self.positions.len() as u64;
        let typ_length = varint::length(typ);
        let id_length = varint::length(self.id);
        let pcount_length = varint::length(pcount);

        let mut buf = vec![0u8; 1 + typ_length + id_length + pcount_length];
        let mut offset = 0;

        buf[offset] = 0x02;

        offset += 1;

        offset += varint::encode_with_offset(typ, &mut buf, offset)?;

        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        varint::encode_with_offset(pcount as u64, &mut buf, offset)?;

        for (lon, lat) in self.positions {
            buf.extend(&lon.to_bytes_le()?);
            buf.extend(&lat.to_bytes_le()?);
        }

        buf.extend(labels);
        return Ok(buf);
    }
}
