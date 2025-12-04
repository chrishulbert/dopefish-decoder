// This file's responsible for parsing the raw file data into types:

use anyhow::Result;
use crate::versions;
use crate::parse_graphics;
use crate::parse_maps;
use crate::export;

pub fn parse(exe: &[u8], graph_data: &[u8], maps: &[u8]) -> Result<()> {
    println!("Parsing...");

    // Extract necessary tables from the exe:
    let offsets = versions::determine(exe.len())?;
    let map_head = &exe[offsets.map_head_offset .. offsets.map_head_offset + offsets.map_head_len];
    let graph_head = &exe[offsets.graph_head_offset .. offsets.graph_head_offset + offsets.graph_head_len];
    let graph_dict = &exe[offsets.graph_dict_offset .. offsets.graph_dict_offset + offsets.graph_dict_len];

    // Parse all the graphics:
    let graphics = parse_graphics::parse(&graph_data, &graph_head, &graph_dict)?;

    // Parse the maps:
    let maps = parse_maps::parse(&maps, map_head)?;

    // Export:
    export::export(&graphics, &maps)?;
    
    Ok(())
}
