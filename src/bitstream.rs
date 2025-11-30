// This is responsible for iterating bits, LSB first.
pub struct BitStream<'a> {
    data: &'a [u8],
    len: usize,
    byte_index: usize,
    bit_mask: u8,
}

impl<'a> BitStream<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitStream { data, len: data.len(), byte_index: 0, bit_mask: 1 }
    }
}

impl<'a> Iterator for BitStream<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_index >= self.len {
            return None;
        }
        let byte = self.data[self.byte_index];
        let is_set = (byte & self.bit_mask) != 0;
        if self.bit_mask == 0x80 {
            self.byte_index += 1;
            self.bit_mask = 1;
        } else {
            self.bit_mask <<= 1;
        }
        Some(is_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitstream() {
        let input: Vec<u8> = vec![0, 1, 128, 254, 255];
        let output: Vec<bool> = BitStream::new(&input).collect();
        assert_eq!(output, vec![
            false, false, false, false, false, false, false, false, // 0
            true, false, false, false, false, false, false, false, // 1
            false, false, false, false, false, false, false, true, // 128
            false, true, true, true, true, true, true, true, // 254
            true, true, true, true, true, true, true, true, // 255
        ]);
    }
}
