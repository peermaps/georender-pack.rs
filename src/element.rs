use desert::ToBytesLE;
use failure::Error;

pub trait GeorenderPackable {
    fn encode_typ(&self) -> Vec<u8> {}
    fn encode_id(&self) -> Vec<u8> {}
    fn encode_label(&self, label: &[u8], buf: &mut [u8], offset: usize) -> usize {}
    fn encode_point(
        &self,
        point: (f64, f64),
        buf: &mut [u8],
        offset: usize,
    ) -> Result<usize, Error> {
    }
    fn parse_tags(&self) -> (u64, Vec<u8>) {}
}

pub trait Element {}

impl<T> GeorenderPackable for T
where
    T: Element,
{
    fn encode_typ(&self) -> Vec<u8> {}
    fn encode_id(&self) -> Vec<u8> {}
    fn encode_label(&self, label: &[u8], buf: &mut [u8], offset: usize) -> usize {
        let mut encoded = 0;
        (0..label.len()).for_each(|i| {
            buf[offset + encoded] = label[i];
            encoded += 1;
        });
        return encoded;
    }
    fn encode_point(
        &self,
        point: (f64, f64),
        buf: &mut [u8],
        offset: usize,
    ) -> Result<usize, Error> {
        let bytes = (point.0 as f32).to_bytes_le()?;
        let mut encoded = 0;
        (0..bytes.len()).for_each(|i| {
            buf[offset + encoded] = bytes[i];
            encoded += 1;
        });
        let bytes = (point.1 as f32).to_bytes_le()?;
        (0..bytes.len()).for_each(|i| {
            buf[offset + encoded] = bytes[i];
            encoded += 1;
        });
        return Ok(encoded);
    }
}
