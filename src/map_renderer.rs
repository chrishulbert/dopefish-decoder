// This is responsible for rendering a map.

use crate::images::Image;
use crate::parse_maps::Map;
use crate::parse_graphics::Graphics;

pub fn render(map: &Map, graphics: &Graphics) -> Image {
    let tile_size: usize = 16;
    let mut map_image = Image::empty(map.width * tile_size, map.height * tile_size);
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            // Background:
            // Have to unwrap the image twice: Once if it's in range, and secondly if there is an image in that slot.
            if let Some(opt_image) = graphics.tiles_16_unmasked.get(tile.background as usize) {
                if let Some(image) = opt_image {
                    draw(image, &mut map_image, x * tile_size, y * tile_size);
                }
            }

            // Foreground:
            if let Some(foreground) = tile.foreground { // Does this tile have a foreground?
                if let Some(opt_image) = graphics.tiles_16_masked.get(foreground as usize) { // Is this in valid range?
                    if let Some(image) = opt_image { // Does this tile slot have an image?
                        draw(image, &mut map_image, x * tile_size, y * tile_size);
                    }
                }
            }

            // TODO sprites?
        }
    }
    map_image
}

fn draw(sprite: &Image, onto: &mut Image, x: usize, y: usize) {
    for sprite_y in 0 .. sprite.height {
        for sprite_x in 0 .. sprite.width {
            let out_offset = (y + sprite_y) * onto.width + (x + sprite_x);
            let colour = sprite.data[sprite_y * sprite.width + sprite_x];
            let alpha = colour & 0xff;
            if alpha == 0xff {
                onto.data[out_offset] = colour;
            } else if alpha == 0 {
                // Do nothing, it's clear.
            } else {
                // Its an mixed opacity... This shouldn't happen in the context of Keen mapping,
                // so simply show the new colour.
                onto.data[out_offset] = colour;
            }
        }
    }
}
