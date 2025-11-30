// This is responsible for converting planar EGA images into RGBA images.

use crate::palette;
use crate::png;

pub struct Image {
    data: Vec<u32>, // 0xrrggbbaa
    width: usize,
    height: usize,
}

// Convert 4-planes ega data.
pub fn parse_ega_rgbi(data: &[u8], width_div_8: usize, height: usize) -> Image {
    let indexed_pixels = combine_planes(&data, width_div_8, height, 4);
    let rgba: Vec<u32> = indexed_pixels.iter().map(|ix| palette::PALETTE[*ix as usize]).collect();
    let width = width_div_8 * 8;
    Image { data: rgba, width, height }
}

// Convert 5-planes masked ega data.
pub fn parse_ega_rgbim(data: &[u8], width_div_8: usize, height: usize) -> Image {
    let indexed_pixels = combine_planes(&data, width_div_8, height, 5);
    fn rgba_from_masked_index(ix: &u8) -> u32 {
        if ix & 1 == 0 { // Is the mask bit on?
            return palette::PALETTE[(ix >> 1) as usize]; // Remove the mask bit for the palette lookup.
        } else {
            return palette::CLEAR;
        }
    }
    let rgba: Vec<u32> = indexed_pixels.iter().map(rgba_from_masked_index).collect();
    let width = width_div_8 * 8;
    Image { data: rgba, width, height }
}

fn combine_planes(data: &[u8], width_div_8: usize, height: usize, planes: usize) -> Vec<u8> {
    let width = width_div_8 * 8;
    let mut indexed_pixels: Vec<u8> = vec![0; width * height];
    let bytes_per_plane = width_div_8 * height;
    for (plane_index, plane) in data.chunks_exact(bytes_per_plane).take(planes).enumerate() {
        let plane_mask = 1 << plane_index;
        let mut out_index = 0;
        for eight_pixels in plane {
            if eight_pixels & 0x80 != 0 { indexed_pixels[out_index + 0] |= plane_mask; }
            if eight_pixels & 0x40 != 0 { indexed_pixels[out_index + 1] |= plane_mask; }
            if eight_pixels & 0x20 != 0 { indexed_pixels[out_index + 2] |= plane_mask; }
            if eight_pixels & 0x10 != 0 { indexed_pixels[out_index + 3] |= plane_mask; }
            if eight_pixels & 0x8 != 0 { indexed_pixels[out_index + 4] |= plane_mask; }
            if eight_pixels & 0x4 != 0 { indexed_pixels[out_index + 5] |= plane_mask; }
            if eight_pixels & 0x2 != 0 { indexed_pixels[out_index + 6] |= plane_mask; }
            if eight_pixels & 0x1 != 0 { indexed_pixels[out_index + 7] |= plane_mask; }
            out_index += 8;
        }
    }
    indexed_pixels
}

impl Image {
    pub fn png(&self) -> Vec<u8> {
        png::encode(self.width as u32, self.height as u32, &self.data)
    }
}
