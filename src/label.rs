use crate::varint;

pub fn scan(buf: &[u8]) -> Result<usize,failure::Error> {
    let mut offset = 0;
    loop {
        let (s,len) = varint::decode(&buf[offset..])?;
        offset += s + (len as usize);
        if len == 0 { break }
    }
    Ok(offset)
}
