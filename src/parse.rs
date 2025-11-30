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
