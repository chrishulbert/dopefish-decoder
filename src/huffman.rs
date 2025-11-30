// This is responsible for huffman decoding.
// https://moddingwiki.shikadi.net/wiki/Huffman_Compression

use crate::bitstream;

pub struct Node {
    left: u8,
    left_is_leaf: bool,
    right: u8,
    right_is_leaf: bool,
}

pub fn parse_dict(data: &[u8]) -> Vec<Node> {
    data.chunks(4).map(|c|
        Node { left: c[0], left_is_leaf: c[1]==0, right: c[2], right_is_leaf: c[3]==0 }
    ).collect()
}

pub fn decompress(data: &[u8], dict: &[Node], desired_length: usize) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    const START_NODE: usize = 254;
    let mut node_index = START_NODE;
    let stream = bitstream::BitStream::new(data);
    for bit in stream {
        let node = &dict[node_index];
        let (value, is_leaf) = if bit { (node.right, node.right_is_leaf) }
                                           else { (node.left,  node.left_is_leaf) };
        if is_leaf {
            output.push(value);
            if output.len() == desired_length { return output } // Decoded enough; skip the leftover bits.
            node_index = START_NODE;
        } else {
            node_index = value as usize;
        }
    }
    output
}
