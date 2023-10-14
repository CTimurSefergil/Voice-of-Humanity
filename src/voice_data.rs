use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub(crate) fn read_voice_data(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}
