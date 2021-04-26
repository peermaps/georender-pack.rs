use crate::varint;
use crate::{label, tags};
use desert::{ToBytesLE, FromBytesLE};
use failure::Error;

#[test]
fn peer_line() -> Result<(),Error> {
    let tags = vec![("source", "bing"), ("highway", "residential")];
    let positions: Vec<f32> = vec![
        31.184799400000003, 29.897739500000004,
        31.184888100000002, 29.898801400000004,
        31.184858400000003, 29.8983899,
    ];
    let id: u64 = 234941233;
    let line = Line::from_tags(id, &tags, &positions)?;

    let bytes = line.to_bytes_le().unwrap();
    assert_eq!(
        "029c03b1d6837003787af941922eef41a77af941bf30ef41977af941e72fef4100",
        hex::encode(&bytes)
    );
    assert_eq!(
        Line::from_bytes_le(&bytes)?,
        (bytes.len(),line)
    );
    Ok(())
}

#[derive(Debug,Clone,PartialEq)]
pub struct Line {
    pub id: u64,
    pub positions: Vec<f32>,
    pub feature_type: u64,
    pub labels: Vec<u8>,
}

impl Line {
    pub fn from_tags(id: u64, tags: &[(&str, &str)], positions: &[f32]) -> Result<Line,Error> {
        let (feature_type, labels) = tags::parse(tags)?;
        Ok(Line {
            id,
            positions: positions.to_vec(),
            feature_type,
            labels
        })
    }
    pub fn new(id: u64, feature_type: u64, labels: &[u8], positions: &[f32]) -> Line {
        Line {
            id,
            feature_type,
            labels: labels.to_vec(),
            positions: positions.to_vec(),
        }
    }
}

impl ToBytesLE for Line {
    fn to_bytes_le(&self) -> Result<Vec<u8>, Error> {
        let pcount = self.positions.len()/2;
        let ft_length = varint::length(self.feature_type);
        let id_length = varint::length(self.id);
        let pcount_length = varint::length(pcount as u64);

        let mut buf =
            vec![0u8; 1 + ft_length + id_length + pcount_length + (2 * 4 * pcount) + self.labels.len()];
        let mut offset = 0;
        buf[offset] = 0x02;
        offset += 1;

        offset += varint::encode(self.feature_type, &mut buf[offset..])?;

        offset += varint::encode(self.id, &mut buf[offset..])?;

        offset += varint::encode(pcount as u64, &mut buf[offset..])?;

        for p in self.positions.iter() {
            offset += p.write_bytes_le(&mut buf[offset..])?;
        }

        buf[offset..].copy_from_slice(&self.labels);
        return Ok(buf);
    }
}

impl FromBytesLE for Line {
    fn from_bytes_le(buf: &[u8]) -> Result<(usize,Self), Error> {
        if buf[0] != 0x02 {
            failure::bail!["parsing line failed. expected 0x02, received 0x{:02x}", buf[0]];
        }
        let mut offset = 1;
        let (s,feature_type) = varint::decode(&buf[offset..])?;
        offset += s;
        let (s,id) = varint::decode(&buf[offset..])?;
        offset += s;

        let (s,pcount) = varint::decode(&buf[offset..])?;
        offset += s;

        let mut positions = Vec::with_capacity((pcount as usize)*2);
        for _ in 0..pcount*2 {
            let (s,x) = f32::from_bytes_le(&buf[offset..])?;
            offset += s;
            positions.push(x);
        }

        let s = label::scan(&buf[offset..])?;
        let labels = buf[offset..offset+s].to_vec();
        offset += s;
        Ok((offset, Self { id, positions, feature_type, labels }))
    }
}
