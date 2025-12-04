// This is responsible for parsing all the maps.

use crate::carmackization;
use crate::rlew;
use anyhow::{Result, bail};

pub fn parse(gamemaps: &[u8], map_head_data: &[u8]) -> Result<Vec<Map>> {
    println!("Parsing maps...");

    // Parse the single map_head from the exe file:
    let map_head = MapHead::parse(map_head_data);

    // Parse the headers for each map from gamemaps.ck*:
    let headers: Vec<Header> = map_head.offsets.iter().map(|offset| {
        let data = &gamemaps[*offset .. *offset+38];
        Header::parse(&data)
    }).collect();

    // Decompress each map:
    let maps: Vec<Map> = headers.into_iter().map(|header| {
        Map::parse(&gamemaps, header, map_head.rlew_key)
    }).collect();

    Ok(maps)
}

// This represents the maphead which is embedded in the exe.
// It points to the location of each map in the gamemaps file.
// Apologies for the confusing naming vs 'map header', as 'map head' is what it's called in all
// the reference docs, so i left it that way for consistency.
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

// This is the header for a single map, from the gamemaps file.
#[derive(Debug)]
struct Header {
    offset_plane_0: usize, // Background plane (unmasked tiles).
    offset_plane_1: usize, // Foreground plane (masked tiles).
    offset_plane_2: usize, // Sprites/info plane.
    len_plane_0: usize, // Length of the compressed data.
    len_plane_1: usize,
    len_plane_2: usize,
    width_tiles: usize,
    height_tiles: usize,
    name: String,
}
impl Header {
    fn parse(data: &[u8]) -> Self {
        Header {
            offset_plane_0: u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize,
            offset_plane_1: u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize,
            offset_plane_2: u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize,
            len_plane_0: u16::from_le_bytes(data[12..14].try_into().unwrap()) as usize,
            len_plane_1: u16::from_le_bytes(data[14..16].try_into().unwrap()) as usize,
            len_plane_2: u16::from_le_bytes(data[16..18].try_into().unwrap()) as usize,
            width_tiles: u16::from_le_bytes(data[18..20].try_into().unwrap()) as usize,
            height_tiles: u16::from_le_bytes(data[20..22].try_into().unwrap()) as usize,
            name: string_from_asciiz(&data[22..]),
        }
    }
}
fn string_from_asciiz(data: &[u8]) -> String {
    let end = data.iter().position(|b| *b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}

// This represents a decompressed map:
pub struct Map {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<MapTile>>, // Think of this as a vec of rows.
}
pub struct MapTile {
    pub background: u16,
    pub foreground: Option<u16>,
    pub _sprite: Option<u16>,
}
impl Map {
    fn parse(gamemaps: &[u8], header: Header, rlew_key: u16) -> Self {
        let plane_0= parse_plane(&gamemaps, &header, 0, rlew_key).unwrap();
        let plane_1= parse_plane(&gamemaps, &header, 1, rlew_key).unwrap();
        let plane_2= parse_plane(&gamemaps, &header, 2, rlew_key).unwrap();
        let tiles = tiles_from_planes(&plane_0, &plane_1, &plane_2, header.width_tiles);
        Map {
            name: header.name,
            width: header.width_tiles,
            height: header.height_tiles,
            tiles,
        }
    }
}

// De-carmacks then de-rlew's the compressed plane then parses to u16s.
fn parse_plane(gamemaps: &[u8], header: &Header, plane: u8, key: u16) -> Result<Vec<u16>> {
    // Expand:
    let offset: usize = match plane {
        0 => header.offset_plane_0,
        1 => header.offset_plane_1,
        _ => header.offset_plane_2,
    };
    let length: usize = match plane {
        0 => header.len_plane_0,
        1 => header.len_plane_1,
        _ => header.len_plane_2,
    };
    let compressed = &gamemaps[offset .. (offset + length)];
    let half_expanded = carmackization::expand_with_length_header(compressed)?;
    let expanded = rlew::expand_with_length_header(&half_expanded, key)?;
    let expected_length = header.width_tiles * header.height_tiles * 2;
    if expanded.len() != expected_length { bail!("Expanded plane data isn't the expected number of tiles!") }
    
    // Convert:
    let parsed: Vec<u16> = expanded.chunks_exact(2).map(|c| {
        (c[0] as u16) + ((c[1] as u16) << 8)
    }).collect();
    Ok(parsed)
}

// Converts expanded plane data into a neat vec of map rows.
fn tiles_from_planes(plane_0: &[u16], plane_1: &[u16], plane_2: &[u16], width: usize) -> Vec<Vec<MapTile>> {
    // Combine the planes.
    let zipped: Vec<(u16, u16, u16)> = plane_0.iter().zip(plane_1).zip(plane_2).map(|z| {
        (*z.0.0, *z.0.1, *z.1)
    }).collect();

    // Split them into rows.
    zipped.chunks_exact(width).map(|row| {
        row.iter().map(|tile| {
            let sprite: Option<u16> = if tile.2 == 0 { None } else { Some(tile.2 - 1) };
            let foreground: Option<u16> = if tile.1 == 0 { None } else { Some(tile.1 - 1) };
            MapTile { background: tile.0, foreground, _sprite: sprite }
        }).collect()
    }).collect()
}