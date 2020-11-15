use crate::{parse_tags, Tags};
use desert::ToBytesLE;
use failure::Error;
use std::rc::Rc;
use varinteger;

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
