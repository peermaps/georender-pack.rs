use crate::varint;
use crate::{label, point, tags};
use desert::ToBytesLE;
use failure::Error;

#[test]
fn peer_line() {
    let tags = vec![("source", "bing"), ("highway", "residential")];
    let positions: Vec<f64> = vec![
        31.184799400000003, 29.897739500000004,
        31.184888100000002, 29.898801400000004,
        31.184858400000003, 29.8983899,
    ];
    let id: u64 = 234941233;
    let line = PeerLine::new(id, &tags, &positions);

    let bytes = line.to_bytes_le().unwrap();
    assert_eq!(
        "02c801b1d6837003787af941922eef41a77af941bf30ef41977af941e72fef4100",
        hex::encode(bytes)
    );
}

#[derive(Debug)]
pub struct PeerLine<'a> {
    pub id: u64,
    pub positions: &'a Vec<f64>,
    pub tags: &'a Vec<(&'a str, &'a str)>,
}

impl<'a> PeerLine<'a> {
    pub fn new(
        id: u64,
        tags: &'a Vec<(&str, &str)>,
        positions: &'a Vec<f64>,
    ) -> PeerLine<'a> {
        return PeerLine {
            id,
            positions,
            tags,
        };
    }
}

impl<'a> ToBytesLE for PeerLine<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, label) = tags::parse(self.tags)?;
        let pcount = self.positions.len()/2;
        let typ_length = varint::length(typ);
        let id_length = varint::length(self.id);
        let pcount_length = varint::length(pcount as u64);

        let mut buf =
            vec![0u8; 1 + typ_length + id_length + pcount_length + (2 * 4 * pcount) + label.len()];
        let mut offset = 0;
        buf[offset] = 0x02;
        offset += 1;

        offset += varint::encode_with_offset(typ, &mut buf, offset)?;

        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        offset += varint::encode_with_offset(pcount as u64, &mut buf, offset)?;

        for p in self.positions.iter() {
            offset += point::encode_with_offset(*p, &mut buf, offset)?;
        }

        label::encode_with_offset(&label, &mut buf, offset);
        return Ok(buf);
    }
}
