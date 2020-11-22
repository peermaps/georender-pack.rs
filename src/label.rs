pub fn encode_with_offset(label: &[u8], buf: &mut [u8], offset: usize) -> usize {
    let mut encoded = 0;
    (0..label.len()).for_each(|i| {
        buf[offset + encoded] = label[i];
        encoded += 1;
    });
    return encoded;
}
