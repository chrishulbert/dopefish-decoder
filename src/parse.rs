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

    // Read the level headers from gamemaps:
    for (i, offset) in map_head.offsets.iter().enumerate() {
        let header_data = &maps[*offset .. *offset+38];
        let header = MapHeader::parse(&header_data);
        println!("{}: {:#?}", i, header);
    }
    
    Ok(())
}

// This represents the maphead which is embedded in the exe.
// It points to the location of each map in the gamemaps file.
#[derive(Debug)]
struct MapHead {
    rlew_key: u16,
    offsets: Vec<usize>, // Raw file offsets to the start of each map in gamemaps.
}
impl MapHead {
    fn parse(data: &[u8]) -> Self {
        let rlew_key: u16 = (data[0] as u16) + ((data[1] as u16) << 8);
        let offsets_data = &data[2..];
        let mut offsets: Vec<usize> = Vec::new();
        for c in offsets_data.chunks_exact(4) {
            let offset = u32::from_le_bytes(c.try_into().unwrap()) as usize;
            if offset == 0 || offset > 9999999 { continue }
            offsets.push(offset);
        }
        MapHead { rlew_key, offsets }
    }
}

// This is the header for a map, from the gamemaps file.
#[derive(Debug)]
struct MapHeader {
    offset_plane_0: u32, // Background plane (unmasked tiles).
    offset_plane_1: u32, // Foreground plane (masked tiles).
    offset_plane_2: u32, // Sprites/info plane.
    len_plane_0: u16,
    len_plane_1: u16,
    len_plane_2: u16,
    width_tiles: u16,
    height_tiles: u16,
    name: String,
}
impl MapHeader {
    fn parse(data: &[u8]) -> Self {
        MapHeader {
            offset_plane_0: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            offset_plane_1: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            offset_plane_2: u32::from_le_bytes(data[8..12].try_into().unwrap()),
            len_plane_0: u16::from_le_bytes(data[12..14].try_into().unwrap()),
            len_plane_1: u16::from_le_bytes(data[14..16].try_into().unwrap()),
            len_plane_2: u16::from_le_bytes(data[16..18].try_into().unwrap()),
            width_tiles: u16::from_le_bytes(data[18..20].try_into().unwrap()),
            height_tiles: u16::from_le_bytes(data[20..22].try_into().unwrap()),
            name: string_from_asciiz(&data[22..]),
        }
    }
}

fn string_from_asciiz(data: &[u8]) -> String {
    let end = data.iter().position(|b| *b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}