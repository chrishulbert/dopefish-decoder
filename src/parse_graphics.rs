// This is responsible for parsing all the graphics.

use crate::images;
use crate::egagraph;
use anyhow::Result;

pub fn parse(graph_data: &[u8], graph_head: &[u8], graph_dict: &[u8]) -> Result<Graphics> {
    println!("Parsing graphics...");

    let mut chunks = egagraph::ChunkIterator::new(&graph_data, &graph_head, &graph_dict)?;
    let mut graphics = Graphics::new();
    
    // Go through the chunks in order:
    let unmasked_picture_table = parse_picture_table(&chunks.next());
    let masked_picture_table = parse_picture_table(&chunks.next());
    let sprite_table = parse_sprite_table(&chunks.next());

    chunks.next(); // Font a.
    chunks.next(); // Font b.
    chunks.next(); // Font c.

    // Unmasked pictures:
    for p in unmasked_picture_table.iter() {
        let data = chunks.next();
        let image: Option<images::Image> =
            if p.width_div_8 == 0 || p.height == 0 || data.len() == 0 {
                None
            } else {
                Some(images::parse_ega_rgbi(&data, p.width_div_8 as usize, p.height as usize))
            };
        graphics.pictures_unmasked.push(image);
    }

    // Masked pictures:
    for p in masked_picture_table.iter() {
        let data = chunks.next();
        let image: Option<images::Image> = 
            if p.width_div_8 == 0 || p.height == 0 || data.len() == 0 {
                None
            } else {
                Some(images::parse_ega_rgbim(&data, p.width_div_8 as usize, p.height as usize))
            };
        graphics.pictures_masked.push(image);
    }

    // Sprites:
    for s in sprite_table.iter() {
        let data = chunks.next();
        let image: Option<images::Image> = 
            if s.width_div_8 == 0 || s.height == 0 || data.len() == 0 {
                None
            } else {
                Some(images::parse_ega_rgbim(&data, s.width_div_8 as usize, s.height as usize))
            };
        graphics.sprites.push(image);
    }

    // Unmasked 8x8 tiles:
    // These are all stored in one chunk that has no length header.
    // These are not used in-game, should we bother?
    let unmasked_tiles_8 = chunks.next_with_auto_length();
    for t in unmasked_tiles_8.chunks_exact(32) {
        let image = images::parse_ega_rgbi(&t, 1, 8);
        graphics.tiles_8_unmasked.push(image);
    }

    // Masked 8x8 tiles:
    // These are stored as above, no length header.
    let masked_tiles_8 = chunks.next_with_auto_length();
    for t in masked_tiles_8.chunks_exact(40) {
        let image = images::parse_ega_rgbim(&t, 1, 8);
        graphics.tiles_8_masked.push(image);
    }

    // Unmasked 16x16 tiles:
    // These get a chunk each but the chunks have no header.
    loop {
        let chunk = chunks.next_with_auto_length();
        let len = chunk.len();
        if 128 <= len && len <= 159 { // Tests a range in case the auto length decoded some extra.
            let image = images::parse_ega_rgbi(&chunk, 2, 16);
            graphics.tiles_16_unmasked.push(Some(image));
        } else if len == 0 { // Empties.
            graphics.tiles_16_unmasked.push(None);
        } else { // Found the first masked one.
            chunks.rewind_once();
            break
        }
    }

    // Masked 16x16 tiles:
    loop {
        let chunk = chunks.next_with_auto_length();
        let len = chunk.len();
        if 160 <= len && len <= 160 + 16 { // 16 masked.
            let image = images::parse_ega_rgbim(&chunk, 2, 16);
            graphics.tiles_16_masked.push(Some(image));
        } else if len == 0 { // Ignore empty chunks.
            graphics.tiles_16_masked.push(None);
        } else {
            break // Finished reading tiles, hit the uninteresting chunks at the end now.
        }
    }

    Ok(graphics)
}

pub struct Graphics {
    pub pictures_unmasked: Vec<Option<images::Image>>,
    pub pictures_masked: Vec<Option<images::Image>>,
    pub sprites: Vec<Option<images::Image>>,
    pub tiles_8_unmasked: Vec<images::Image>,
    pub tiles_8_masked: Vec<images::Image>,
    pub tiles_16_unmasked: Vec<Option<images::Image>>,
    pub tiles_16_masked: Vec<Option<images::Image>>,
}
impl Graphics {
    fn new() -> Self {
        Graphics {
            pictures_unmasked: Vec::new(),
            pictures_masked: Vec::new(),
            sprites: Vec::new(),
            tiles_8_unmasked: Vec::new(),
            tiles_8_masked: Vec::new(),
            tiles_16_unmasked: Vec::new(),
            tiles_16_masked: Vec::new(),
        }
    }
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
    _x_offset: u32,
    _y_offset: u32,
    _clip_left: u32,
    _clip_top: u32,
    _clip_right: u32,
    _clip_bottom: u32,
    _shifts: u32,
}
fn parse_sprite_table(data: &[u8]) -> Vec<SpriteTableEntry> {
    // This uses chunks_exact instead of chunks, because the masked picture table isn't the right length.
    data.chunks_exact(18).map(|c| SpriteTableEntry {
        width_div_8: c[0] as u32 + ((c[1] as u32) << 8),
        height: c[2] as u32 + ((c[3] as u32) << 8),
        _x_offset: c[4] as u32 + ((c[5] as u32) << 8),
        _y_offset: c[6] as u32 + ((c[7] as u32) << 8),
        _clip_left: c[8] as u32 + ((c[9] as u32) << 8),
        _clip_top: c[10] as u32 + ((c[11] as u32) << 8),
        _clip_right: c[12] as u32 + ((c[13] as u32) << 8),
        _clip_bottom: c[14] as u32 + ((c[15] as u32) << 8),
        _shifts: c[16] as u32 + ((c[17] as u32) << 8),
    }).collect()
}
