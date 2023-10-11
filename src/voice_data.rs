use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn read_voice_data(filepath: &str, x: usize) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut voices = Vec::new();

    for line in reader.lines() {
        &voices.push(line?);
    }

    Ok(voices[x].clone())
}
