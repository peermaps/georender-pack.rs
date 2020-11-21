use crate::varint;
use crate::{parse_tags, point, Tags};
use desert::ToBytesLE;
use earcutr;
use failure::Error;
use std::rc::Rc;

#[test]
fn peer_area() {
    let tags = vec![
        ("source", "bing"),
        ("boundary", "protected_area"),
        ("tiger:cfcc", "A41"),
    ];
    let positions: Vec<(f64, f64)> = vec![
        (31.184799400000003, 29.897739500000004),
        (31.184888100000002, 29.898801400000004),
        (31.184858400000003, 29.8983899),
        (31.184799400000003, 29.897739500000004),
    ];
    let id: u64 = 234941233;
    let line = PeerArea::new(id, &tags, &positions);

    let bytes = line.to_bytes_le().unwrap();
    assert_eq!(
        "03ae01b1d6837004787af941922eef41a77af941bf30ef41977af941e72fef41787af941922eef410301020000",
        hex::encode(bytes)
    );
}

#[derive(Debug)]
pub struct PeerArea<'a> {
    pub id: u64,
    pub positions: &'a Vec<(f64, f64)>,
    pub tags: Rc<Tags<'a>>,
}

impl<'a> PeerArea<'a> {
    pub fn new(
        id: u64,
        tags: &'a Vec<(&str, &str)>,
        positions: &'a Vec<(f64, f64)>,
    ) -> PeerArea<'a> {
        let tags = Tags { iter: tags };
        return PeerArea {
            id: id,
            positions: positions,
            tags: Rc::new(tags),
        };
    }
}

fn earcut(positions: &Vec<(f64, f64)>) -> Vec<usize> {
    let mut coords: Vec<f64> = vec![0.0; positions.len() * 2];
    let mut offset = 0;
    while offset < positions.len() {
        let p = positions[offset];
        coords[offset] = p.0;
        offset += 1;
        coords[offset] = p.1;
        offset += 1;
    }

    return earcutr::earcut(&coords, &vec![], 2);
}

impl<'a> ToBytesLE for PeerArea<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, labels) = parse_tags(&self.tags)?;
        let pcount = self.positions.len();
        let typ_length = varint::length(typ);
        let id_length = varint::length(self.id);
        let pcount_length = varint::length(pcount as u64);

        let cells = earcut(&self.positions);
        let clen = varint::length((cells.len() / 3) as u64);
        let clen_data = cells
            .iter()
            .fold(0, |acc, c| acc + varint::length(*c as u64));

        let mut buf =
            vec![
                0u8;
                1 + typ_length + id_length + pcount_length + (2 * 4 * pcount) + clen + clen_data
            ];

        let mut offset = 0;
        buf[offset] = 0x03;

        offset += 1;
        offset += varint::encode_with_offset(typ, &mut buf, offset)?;
        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;
        offset += varint::encode_with_offset(pcount as u64, &mut buf, offset)?;

        // positions
        for (lon, lat) in self.positions {
            offset += point::encode_with_offset(*lon, &mut buf, offset)?;
            offset += point::encode_with_offset(*lat, &mut buf, offset)?;
        }

        offset += varint::encode_with_offset(cells.len() as u64, &mut buf, offset)?;

        // cells
        for &cell in cells.iter() {
            offset += varint::encode_with_offset(cell as u64, &mut buf, offset)?;
        }

        buf.extend(labels);
        return Ok(buf);
    }
}
