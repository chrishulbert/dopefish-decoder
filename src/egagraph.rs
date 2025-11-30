// This is responsible for splitting the EGAGRAPH into decompressed chunks.

use crate::huffman;
use anyhow::{Result, bail};

// Wraps the data+head+dict together to make a kinda-iterator that returns all the chunks.
// It's not quite an iterator because it has an extra 'next' function for tiles with a hardcoded length.
pub struct ChunkIterator<'a> {
    data: &'a [u8],
    chunk_offsets: Vec<usize>, // aka 'graph_head' without the last value that points past the end of file.
    dict: Vec<huffman::Node>,
    index: usize,
}

impl<'a> ChunkIterator<'a> {
    pub fn new(data: &'a [u8], head: &[u8], dict: &[u8]) -> Result<Self> {
        let chunk_offsets = parse_graph_head(&head, data.len())?;
        let dict = huffman::parse_dict(dict);
        Ok(ChunkIterator { data, chunk_offsets, dict, index: 0 })
    }
}

impl<'a> ChunkIterator<'a> {
    // This decompresses the usual case where the chunk has a length header.
    pub fn next(&mut self) -> Vec<u8> {
        let offset = self.chunk_offsets[self.index];
        self.index += 1;
        if offset == 0xffffff { return vec![] } // Empty chunk.
        let chunk = &self.data[offset..];
        let len = u32::from_le_bytes(chunk[0..4].try_into().unwrap()) as usize;
        let compressed = &chunk[4..];
        huffman::decompress(compressed, &self.dict, len)
    }

    // This decompresses tiles, whose chunks have hardcoded lengthsd and no length header.
    pub fn next_with_length(&mut self, desired_length: usize) -> Vec<u8> {
        let offset = self.chunk_offsets[self.index];
        self.index += 1;
        if offset == 0xffffff { return vec![] }
        let chunk = &self.data[offset..];
        huffman::decompress(chunk, &self.dict, desired_length)
    }
}

// https://moddingwiki.shikadi.net/wiki/EGAGraph_Format
// Graph head is an array of 3-byte little-endian offsets.
// This doesn't return the last one, as it is only used for validation and isn't the start of a chunk.
// This also validates it.
fn parse_graph_head(data: &[u8], graph_data_len: usize) -> Result<Vec<usize>> {
    if data.len()%3 != 0 { bail!("Graph head isn't an even multiple of 3 bytes.") }
    let mut values: Vec<usize> = data
        .chunks_exact(3)
        .map(|c| (c[0] as usize) + ((c[1] as usize) << 8) + ((c[2] as usize) << 16))
        .collect();
    if values[0] != 0 { bail!("Graph head does not start with 0!") }
    if *values.last().unwrap() != graph_data_len { bail!("Graph head does not match the graph file size!") }
    values.pop(); // Remove the final one which points to the end of file.
    Ok(values)
}
