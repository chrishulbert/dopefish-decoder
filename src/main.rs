use anyhow::Result;

mod read;
mod parse;
mod parse_graphics;
mod versions;
mod palette;
mod huffman;
mod bitstream;
mod png;
mod images;
mod egagraph;
mod export;
mod carmackization;
mod rlew;

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
