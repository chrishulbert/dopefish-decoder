use anyhow::Result;

mod bitstream;
mod carmackization;
mod egagraph;
mod export;
mod huffman;
mod images;
mod map_renderer;
mod palette;
mod parse_graphics;
mod parse_maps;
mod parse;
mod png;
mod read;
mod rlew;
mod versions;

fn main() -> Result<()>{
    let args: Vec<String> = std::env::args().collect();
    
    println!("-=[ Dopefish Decoder ]=-");
    if args.len() < 4 {
        println!("Usage:");
        println!("dopefish-decoder /Path/To/Keen456.exe /Foo/EGAGRAPH.CK456 GAMEMAPS.CK456");
    } else {
        read::read(&args[1], &args[2], &args[3])?;
    }
    Ok(())
}
