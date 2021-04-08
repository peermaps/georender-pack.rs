use crate::varint;
use crate::{label, point, tags};
use desert::ToBytesLE;
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
    let line = PeerLine::from_tags(id, &tags, &positions)?;

    let bytes = line.to_bytes_le().unwrap();
    assert_eq!(
        "029c03b1d6837003787af941922eef41a77af941bf30ef41977af941e72fef4100",
        hex::encode(bytes)
    );
    Ok(())
}

#[derive(Debug)]
pub struct PeerLine {
    pub id: u64,
    pub positions: Vec<f32>,
    pub feature_type: u64,
    pub labels: Vec<u8>,
}

impl PeerLine {
    pub fn from_tags(id: u64, tags: &[(&str, &str)], positions: &[f32]) -> Result<PeerLine,Error> {
        let (feature_type, labels) = tags::parse(tags)?;
        Ok(PeerLine {
            id,
            positions: positions.to_vec(),
            feature_type,
            labels
        })
    }
    pub fn new(id: u64, feature_type: u64, labels: &[u8], positions: &[f32]) -> PeerLine {
        PeerLine {
            id,
            feature_type,
            labels: labels.to_vec(),
            positions: positions.to_vec(),
        }
    }
}

impl ToBytesLE for PeerLine {
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

        offset += varint::encode_with_offset(self.feature_type, &mut buf, offset)?;

        offset += varint::encode_with_offset(self.id, &mut buf, offset)?;

        offset += varint::encode_with_offset(pcount as u64, &mut buf, offset)?;

        for p in self.positions.iter() {
            offset += point::encode_with_offset(*p, &mut buf, offset)?;
        }

        label::encode_with_offset(&self.labels, &mut buf, offset);
        return Ok(buf);
    }
}
