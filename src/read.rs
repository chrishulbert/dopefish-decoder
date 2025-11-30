// This file's responsible for reading the game from disk, then passing onto the next step.

use std::fs;
use anyhow::Result;
use crate::parse;

pub fn read(exe: &str, graph: &str, maps: &str) -> Result<()> {
    println!("Reading...");
    
    println!("Executable: {}", exe);
    let exe_buf = fs::read(exe)?;

    println!("Graphics: {}", graph);
    let graph_buf = fs::read(graph)?;

    println!("Maps: {}", maps);
    let maps_buf = fs::read(maps)?;
    
    parse::parse(&exe_buf, &graph_buf, &maps_buf)?;
    
    Ok(())
}
