use crate::varint;
use crate::{parse_tags, point};
use desert::ToBytesLE;
use failure::Error;

#[test]
fn peer_line() {
    let tags = vec![("source", "bing"), ("highway", "residential")];
    let positions: Vec<(f64, f64)> = vec![
        (31.184799400000003, 29.897739500000004),
        (31.184888100000002, 29.898801400000004),
        (31.184858400000003, 29.8983899),
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
    pub positions: &'a Vec<(f64, f64)>,
    pub typ: u64,
    pub label: Vec<u8>,
}

impl<'a> PeerLine<'a> {
    pub fn new(
        id: u64,
        tags: &'a Vec<(&str, &str)>,
        positions: &'a Vec<(f64, f64)>,
    ) -> PeerLine<'a> {
        let (typ, label) = parse_tags(tags);
        return PeerLine {
            id: id,
            positions: positions,
            typ: typ,
            label: label,
        };
    }
}

impl<'a> ToBytesLE for PeerLine<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let pcount = self.positions.len();
        let typ_length = varint::length(self.typ);
        let id_length = varint::length(self.id);
        let pcount_length = varint::length(pcount as u64);

        let mut buf = vec![0u8; 1 + typ_length + id_length + pcount_length + (2 * 4 * pcount)];
        let mut offset = 0;
        buf[offset] = 0x02;
        offset += 1;

        offset += varint::encode_with_offset(self.typ, &mut buf, offset)?;

        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        offset += varint::encode_with_offset(pcount as u64, &mut buf, offset)?;

        for (lon, lat) in self.positions {
            offset += point::encode_with_offset(*lon, &mut buf, offset)?;
            offset += point::encode_with_offset(*lat, &mut buf, offset)?;
        }

        buf.extend(&self.label);
        return Ok(buf);
    }
}
