use std::{fmt::Error, io};
use anyhow::Result;

fn main() -> Result<()> {
    // let file = File::open("input.txt")?;
    // let reader = BufReader::new(file);
    let input = include_str!("input.txt");
    let lines = input.lines();
    Ok(())
}

56