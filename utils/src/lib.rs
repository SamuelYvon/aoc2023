use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

pub trait Problem {
    fn get_part1(&self) -> fn(bool) -> ();
    fn get_part2(&self) -> fn(bool) -> ();
}

/// Read a file line by line, clearing the empty ones.
pub fn lines_of_file(path: &str) -> Result<Vec<String>> {
    let file = File::open(Path::new(path));
    let mut buff = String::new();
    file?
        .read_to_string(&mut buff)
        .map_err(|_| std::io::ErrorKind::InvalidData)?;

    let ret: Vec<String> = buff
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(ret)
}
