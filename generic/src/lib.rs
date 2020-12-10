
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// warning last empty line will nog be read!!!!
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
