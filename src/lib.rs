use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn read_puzzle_input<P: AsRef<Path>>(fp: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>{
    let file = File::open(fp)?;
    Ok(io::BufReader::new(file).lines())
}