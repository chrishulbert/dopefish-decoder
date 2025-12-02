// This is responsible for exporting the assets to eg pngs.

use crate::parse_graphics;
use crate::parse_maps;
use crate::images;
use anyhow::Result;

pub fn export(graphics: &parse_graphics::Graphics, maps: &[parse_maps::Map]) -> Result<()> {
    println!("Exporting assets...");

    export_optionals(&graphics.pictures_unmasked, "OutputPictureUnmasked")?;
    export_optionals(&graphics.pictures_masked, "OutputPictureMasked")?;
    export_optionals(&graphics.sprites, "OutputSprite")?;
    export_images(&graphics.tiles_8_unmasked, "OutputTile8Unmasked")?;
    export_images(&graphics.tiles_8_masked, "OutputTile8Masked")?;
    export_optionals(&graphics.tiles_16_unmasked, "OutputTile16Unmasked")?;
    export_optionals(&graphics.tiles_16_masked, "OutputTile16Masked")?;

    for (index, map) in maps.iter().enumerate() {
        let image = crate::map_renderer::render(&map, &graphics);
        let png = image.png();
        let path = format!("OutputMap{} - {}.png", index, map.name);
        std::fs::write(path, &png)?;
    }

    Ok(())
}

fn export_images(images: &[images::Image], prefix: &str) -> Result<()> {
    for (index, image) in images.iter().enumerate() {
        let png = image.png();
        let path = format!("{}{}.png", prefix, index);
        std::fs::write(path, &png)?;
    }
    Ok(())
}

fn export_optionals(images: &[Option<images::Image>], prefix: &str) -> Result<()> {
    for (index, image) in images.iter().enumerate() {
        let Some(image) = image else { continue };
        let png = image.png();
        let path = format!("{}{}.png", prefix, index);
        std::fs::write(path, &png)?;
    }
    Ok(())
}
