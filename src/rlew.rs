// This file is responsible for expanding RLEW-compressed data eg maps.
// https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression
// https://github.com/camoto-project/gamecompjs/blob/master/formats/cmp-rlew-id.js
// https://github.com/gerstrong/Commander-Genius/blob/master/src/fileio/compression/CRLE.cpp

use anyhow::{Result, bail};

pub fn expand(compressed: &[u8], key: u16) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    let mut bytes = compressed.iter();
    loop {
        // Read the word.
        let Some(le) = bytes.next() else {
            break // Reached EOF with an even number of bytes.
        };
        let Some(be) = bytes.next() else {
            out.push(*le); // Reached EOF with an odd number of bytes.
            break
        };
        let word = (*le as u16) + ((*be as u16) << 8);

        if word == key { // Repeater.
            let Some(count_le) = bytes.next() else { bail!("Count le byte missing after RLEW key!") };
            let Some(count_be) = bytes.next() else { bail!("Count be byte missing after RLEW key!") };
            let Some(value_le) = bytes.next() else { bail!("Value le byte missing after RLEW key!") };
            let Some(value_be) = bytes.next() else { bail!("Value be byte missing after RLEW key!") };
            let count = (*count_le as u16) + ((*count_be as u16) << 8);
            for _ in 0..count {
                out.push(*value_le);
                out.push(*value_be);
            }
        } else {
            out.push(*le);
            out.push(*be);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_repeat_odd_length() {
        let input: Vec<u8> = vec![12, 34, 56, 78, 90];
        let output = expand(&input, 4321).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn test_with_repeat() {
        let input: Vec<u8>    = vec![0x12, 0x34,  0x56, 0x78,  0x11, 0x22,  0x03, 0x00,  0x33, 0x44,  0x9a, 0xbc];
        let expected: Vec<u8> = vec![0x12, 0x34,  0x56, 0x78,  0x33, 0x44,  0x33, 0x44,  0x33, 0x44,  0x9a, 0xbc];
        let output = expand(&input, 0x2211).unwrap();
        assert_eq!(output, expected);
    }
}
