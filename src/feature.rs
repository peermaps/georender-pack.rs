use crate::{Point,Line,Area};
use desert::{FromBytesLE,ToBytesLE};

#[derive(Debug,Clone,PartialEq)]
pub enum Feature {
    Point(Point),
    Line(Line),
    Area(Area)
}

pub fn decode(buf: &[u8]) -> Result<Feature,failure::Error> {
    Ok(Feature::from_bytes_le(buf)?.1)
}

impl FromBytesLE for Feature {
    fn from_bytes_le(buf: &[u8]) -> Result<(usize,Self),failure::Error> {
        if buf.is_empty() { failure::bail!["not enough bytes to decode feature"] }
        Ok(match buf[0] {
            0x01 => {
                let (s,point) = Point::from_bytes_le(buf)?;
                (s,Feature::Point(point))
            },
            0x02 => {
                let (s,line) = Line::from_bytes_le(buf)?;
                (s,Feature::Line(line))
            },
            0x03 => {
                let (s,area) = Area::from_bytes_le(buf)?;
                (s,Feature::Area(area))
            },
            x => {
                failure::bail!["cannot decode feature type. expected 0x01, 0x02, or 0x03. \
                    received {}", x]
            }
        })
    }
}

impl ToBytesLE for Feature {
    fn to_bytes_le(&self) -> Result<Vec<u8>,failure::Error> {
        match self {
            Self::Point(point) => point.to_bytes_le(),
            Self::Line(line) => line.to_bytes_le(),
            Self::Area(area) => area.to_bytes_le(),
        }
    }
}
