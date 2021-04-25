use crate::varint;

pub fn encode_with_offset(label: &[u8], buf: &mut [u8], offset: usize) -> usize {
    let mut encoded = 0;
    (0..label.len()).for_each(|i| {
        buf[offset + encoded] = label[i];
        encoded += 1;
    });
    return encoded;
}

pub fn scan(buf: &[u8]) -> Result<usize,failure::Error> {
    let mut offset = 0;
    loop {
        let (s,len) = varint::decode(&buf[offset..])?;
        offset += s + (len as usize);
        if len == 0 { break }
    }
    Ok(offset)
}
