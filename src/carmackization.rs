// This is responsible for expanding carmackized data.
// https://moddingwiki.shikadi.net/wiki/Carmack_compression
// https://github.com/camoto-project/gamecompjs/blob/master/formats/cmp-carmackize.js
// https://github.com/gerstrong/Commander-Genius/blob/master/src/fileio/compression/CCarmack.cpp

use anyhow::{Result, bail};

pub fn expand(compressed: &[u8]) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    let mut bytes = compressed.iter();
    loop {
        // Read the uint16.
        let Some(count) = bytes.next() else {
            break // Reached EOF with an even number of bytes.
        };
        let Some(tag) = bytes.next() else {
            out.push(*count); // Reached EOF with an odd number of bytes.
            break
        }; 
        // Is it a pointer or literal?
        if *tag == 0xA7 { // Near pointer.
            let Some(distance) = bytes.next() else { bail!("Distance byte missing after near pointer!") };
            if *count == 0 { // Escape.
                out.push(*distance);
                out.push(*tag);
            } else {
                let start = out.len() - (*distance as usize) * 2;
                let len = (*count as usize) * 2;
                let end = start + len;
                out.extend_from_within(start..end);
            }
        } else if *tag == 0xA8 { // Far pointer.
            if *count == 0 { // Escape.
                let Some(escapee) = bytes.next() else { bail!("Low byte missing after escaped far pointer!") };
                out.push(*escapee);
                out.push(*tag);
            } else {
                let Some(offset_le) = bytes.next() else { bail!("Far pointer offset low byte missing!") };
                let Some(offset_be) = bytes.next() else { bail!("Far pointer offset high byte missing!") };
                let offset = (*offset_le as usize) + ((*offset_be as usize) << 8);
                let start = offset * 2;
                let len = (*count as usize) * 2;
                let end = start + len;
                out.extend_from_within(start..end);
            }
        } else { // Literal.
            out.push(*count);
            out.push(*tag);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expand() {
        let input: Vec<u8> = vec![0, 0xA7, 0x12, 0xEE, 0xFF, 0, 0xA8, 0x34, 0xCC, 0xDD];
        let expected: Vec<u8> = vec![0x12, 0xA7, 0xEE, 0xFF, 0x34, 0xA8, 0xCC, 0xDD];
        let output = expand(&input).unwrap();
        assert_eq!(output, expected);
    }
}
