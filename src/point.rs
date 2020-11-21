use desert::ToBytesLE;
use failure::Error;

pub fn encode_with_offset(point: f64, buf: &mut [u8], offset: usize) -> Result<usize, Error> {
    let bytes = (point as f32).to_bytes_le()?;
    let mut encoded = 0;
    (0..bytes.len()).for_each(|i| {
        if let Some(elem) = bytes.get(i) {
            buf[offset + encoded] = *elem;
            encoded += 1;
        }
    });
    return Ok(encoded);
}
