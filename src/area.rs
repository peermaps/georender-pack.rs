use crate::varint;
use crate::{label, point, tags};
use desert::ToBytesLE;
use earcutr;
use failure::Error;

#[test]
fn peer_area() {
    let tags = vec![
        ("source", "bing"),
        ("boundary", "protected_area"),
        ("tiger:cfcc", "A41"),
    ];
    let positions: Vec<f64> = vec![
        31.184799400000003, 29.897739500000004,
        31.184888100000002, 29.898801400000004,
        31.184858400000003, 29.8983899,
    ];
    let id: u64 = 234941233;
    let mut area = PeerArea::new(id, &tags);
    area.push(&positions, &vec![]);

    let bytes = area.to_bytes_le().unwrap();
    assert_eq!(
        // verified against js decoder:
        "03ae01b1d6837003787af941922eef41a77af941bf30ef41977af941e72fef410101000200",
        hex::encode(bytes)
    );
}

#[derive(Debug)]
pub struct PeerArea<'a> {
    pub id: u64,
    pub tags: &'a Vec<(&'a str, &'a str)>,
    pub positions: Vec<f64>,
    pub cells: Vec<usize>,
}

impl<'a> PeerArea<'a> {
    pub fn new(
        id: u64,
        tags: &'a Vec<(&str, &str)>,
    ) -> PeerArea<'a> {
        Self { id, tags, positions: vec![], cells: vec![] }
    }
    pub fn push(&mut self, positions: &[f64], holes: &[usize]) -> () {
        let cells = earcutr::earcut(&positions.to_vec(), &holes.to_vec(), 2);
        let offset = self.positions.len() / 2;
        self.cells.extend(cells.iter().map(|c| c+offset).collect::<Vec<usize>>());
        self.positions.extend_from_slice(positions);
    }
}

impl<'a> ToBytesLE for PeerArea<'a> {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let (typ, label) = tags::parse(&self.tags)?;
        let pcount = self.positions.len()/2;
        let typ_length = varint::length(typ);
        let id_length = varint::length(self.id);
        let pcount_length = varint::length(pcount as u64);
        let clen = varint::length((self.cells.len() / 3) as u64);
        let clen_data = self.cells.iter()
            .fold(0, |acc, c| acc + varint::length(*c as u64));

        let mut buf = vec![
            0u8;
            1 + typ_length
                + id_length
                + pcount_length
                + (2 * 4 * pcount)
                + clen
                + clen_data
                + label.len()
        ];

        let mut offset = 0;
        buf[offset] = 0x03;

        offset += 1;
        offset += varint::encode_with_offset(typ, &mut buf, offset)?;
        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;
        offset += varint::encode_with_offset(pcount as u64, &mut buf, offset)?;

        // positions
        for p in self.positions.iter() {
            offset += point::encode_with_offset(*p, &mut buf, offset)?;
        }

        offset += varint::encode_with_offset((self.cells.len()/3) as u64, &mut buf, offset)?;

        // cells
        for &cell in self.cells.iter() {
            offset += varint::encode_with_offset(cell as u64, &mut buf, offset)?;
        }

        label::encode_with_offset(&label, &mut buf, offset);
        return Ok(buf);
    }
}
