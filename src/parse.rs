// This file's responsible for parsing the raw file data into types:

use anyhow::Result;
use crate::versions;
use crate::parse_graphics;
use crate::export;

pub fn parse(exe: &[u8], graph_data: &[u8], maps: &[u8]) -> Result<()> {
    println!("Parsing...");

    // Extract necessary tables from the exe:
    let offsets = versions::determine(exe.len())?;
    let map_head_data = &exe[offsets.map_head_offset .. offsets.map_head_offset + offsets.map_head_len];
    let graph_head = &exe[offsets.graph_head_offset .. offsets.graph_head_offset + offsets.graph_head_len];
    let graph_dict = &exe[offsets.graph_dict_offset .. offsets.graph_dict_offset + offsets.graph_dict_len];

    // Parse all the graphics:
    let graphics = parse_graphics::parse(&graph_data, &graph_head, &graph_dict)?;
    export::export(&graphics)?;

    // Parse the maps:
    let map_head = MapHead::parse(map_head_data);
    println!("{:#?}", map_head);
    
    Ok(())
}

#[derive(Debug)]
struct MapHead {
    rlew_key: u16,
    offsets: Vec<u32>, // Raw file offsets to the start of each map in gamemaps.
}

impl MapHead {
    fn parse(data: &[u8]) -> Self {
        let rlew_key: u16 = (data[0] as u16) + ((data[1] as u16) << 8);
        let offsets_data = &data[2..];
        let mut offsets: Vec<u32> = Vec::new();
        for c in offsets_data.chunks_exact(4) {
            let offset = u32::from_le_bytes(c.try_into().unwrap());
            if offset == 0 || offset > 9999999 { continue }
            offsets.push(offset);
        }
        MapHead { rlew_key, offsets }
    }
}