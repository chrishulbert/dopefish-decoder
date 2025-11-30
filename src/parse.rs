// This file's responsible for parsing the raw file data into types:

use anyhow::Result;
use crate::versions;
use crate::images;
use crate::egagraph;

pub fn parse(exe: &[u8], graph_data: &[u8], maps: &[u8]) -> Result<()> {
    println!("Parsing...");

    let offsets = versions::determine(exe.len())?;
    let map_head = &exe[offsets.map_head_offset .. offsets.map_head_offset + offsets.map_head_len];
    let graph_head = &exe[offsets.graph_head_offset .. offsets.graph_head_offset + offsets.graph_head_len];
    let graph_dict = &exe[offsets.graph_dict_offset .. offsets.graph_dict_offset + offsets.graph_dict_len];
    
    let mut chunks = egagraph::ChunkIterator::new(graph_data, &graph_head, &graph_dict)?;
    
    let picture_table = parse_picture_table(&chunks.next());
    let masked_picture_table = parse_picture_table(&chunks.next());
    let sprite_table = parse_sprite_table(&chunks.next());
    chunks.next(); // Font a
    chunks.next(); // Font b
    chunks.next(); // Font c
    for (i, p) in picture_table.iter().enumerate() {
        let data = chunks.next();
        if p.width_div_8 == 0  || p.height == 0 || data.len() == 0 { continue }
        let image = images::parse_ega_rgbi(&data, p.width_div_8 as usize, p.height as usize);
        let png = image.png();
        let path = format!("OutputPicture{}.png", i);
        std::fs::write(path, &png)?;
    }
    for (i, p) in masked_picture_table.iter().enumerate() {
        let data = chunks.next();
        if p.width_div_8 == 0  || p.height == 0 || data.len() == 0 { continue }
        let image = images::parse_ega_rgbim(&data, p.width_div_8 as usize, p.height as usize);
        let png = image.png();
        let path = format!("OutputMaskedPicture{}.png", i);
        std::fs::write(path, &png)?;
    }
    for (i, s) in sprite_table.iter().enumerate() {
        let data = chunks.next();
        if s.width_div_8 == 0 || s.height == 0 || data.len() == 0 { continue }
        let image = images::parse_ega_rgbim(&data, s.width_div_8 as usize, s.height as usize);
        let png = image.png();
        let path = format!("OutputSprite{}.png", i);
        std::fs::write(path, &png)?;
    }
    // Unmasked 8x8 tiles are all stored in one chunk that has no length header.
    // These are not used in-game, should we bother?
    let unmasked_tiles_8 = chunks.next_with_auto_length();
    let unmasked_tiles_8_count = unmasked_tiles_8.len() / 32; // Round down because auto length chunk may have extra byte(s).
    for i in 0..unmasked_tiles_8_count {
        let offset = i * 32;
        let image = images::parse_ega_rgbi(&unmasked_tiles_8[offset..], 1, 8);
        let png = image.png();
        let path = format!("OutputTiles8Unmasked{}.png", i);
        std::fs::write(path, &png)?;
    }
    // Masked tiles are stored as above, no length header.
    let masked_tiles_8 = chunks.next_with_auto_length();
    let masked_tiles_8_count = masked_tiles_8.len() / 40;
    for i in 0..masked_tiles_8_count {
        let offset = i * 40;
        let image = images::parse_ega_rgbim(&masked_tiles_8[offset..], 1, 8);
        let png = image.png();
        let path = format!("OutputTiles8Masked{}.png", i);
        std::fs::write(path, &png)?;
    }
    // Now get the 16x16 tiles. These get a chunk each but it has no header.
    // Unmasked first, then masked.
    let mut tile_16_masked_number = 0;
    let mut tile_16_unmasked_number = 0;
    loop {
        if chunks.is_at_end() { break }
        let chunk = chunks.next_with_auto_length();
        let len = chunk.len();
        println!("16 len: {}", len);
        if 128 <= len && len <= 159 { // 16 unmasked. Tests a range in case the auto length decoded some extra.
            let image = images::parse_ega_rgbi(&chunk, 2, 16);
            let png = image.png();
            let path = format!("OutputTiles16Unmasked{}.png", tile_16_unmasked_number);
            std::fs::write(path, &png)?;
            tile_16_unmasked_number += 1;
        } else if 160 <= len && len <= 160 + 16 { // 16 masked.
            let image = images::parse_ega_rgbim(&chunk, 2, 16);
            let png = image.png();
            let path = format!("OutputTiles16Masked{}.png", tile_16_masked_number);
            std::fs::write(path, &png)?;
            tile_16_masked_number += 1;
        } else if len == 0 {
            // Ignore but increment the count?
        } else {
            break // Finished reading tiles, hit the uninteresting chunks at the end now.
        }
    }
    Ok(())
}

struct PictureTableEntry {
    width_div_8: u32,
    height: u32,
}

fn parse_picture_table(data: &[u8]) -> Vec<PictureTableEntry> {
    // This uses chunks_exact instead of chunks, because the masked picture table isn't the right length.
    data.chunks_exact(4).map(|c| PictureTableEntry {
        width_div_8: c[0] as u32 + ((c[1] as u32) << 8),
        height: c[2] as u32 + ((c[3] as u32) << 8),
    }).collect()
}

#[derive(Debug)]
struct SpriteTableEntry {
    width_div_8: u32,
    height: u32,
    x_offset: u32,
    y_offset: u32,
    clip_left: u32,
    clip_top: u32,
    clip_right: u32,
    clip_bottom: u32,
    shifts: u32,
}

fn parse_sprite_table(data: &[u8]) -> Vec<SpriteTableEntry> {
    // This uses chunks_exact instead of chunks, because the masked picture table isn't the right length.
    data.chunks_exact(18).map(|c| SpriteTableEntry {
        width_div_8: c[0] as u32 + ((c[1] as u32) << 8),
        height: c[2] as u32 + ((c[3] as u32) << 8),
        x_offset: c[4] as u32 + ((c[5] as u32) << 8),
        y_offset: c[6] as u32 + ((c[7] as u32) << 8),
        clip_left: c[8] as u32 + ((c[9] as u32) << 8),
        clip_top: c[10] as u32 + ((c[11] as u32) << 8),
        clip_right: c[12] as u32 + ((c[13] as u32) << 8),
        clip_bottom: c[14] as u32 + ((c[15] as u32) << 8),
        shifts: c[16] as u32 + ((c[17] as u32) << 8),
    }).collect()
}
