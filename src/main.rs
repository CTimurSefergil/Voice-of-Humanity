mod voice_data;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Read;
use csv;
use serde;
use rodio::{Decoder, OutputStream, source::Source};
use crate::voice_data::read_voice_data;

fn main() {
// Voices that are going to be used for tests
    let voice1 = read_voice_data("voices.csv", 0).unwrap();
    let voice2 = read_voice_data("voices.csv", 1).unwrap();
// Get system sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// Load a sound from a file, using a path relative to Cargo.toml
    let file1 = BufReader::new(File::open(voice1).unwrap());
    let file2 = BufReader::new(File::open(voice2).unwrap());
// Decode that sound file into a source
    let source1 = Decoder::new(file1).unwrap();
    let source2 = Decoder::new(file2).unwrap();
// Play the sound directly on the device
    stream_handle.play_raw(source1.convert_samples()).expect("TODO: panic message");
    stream_handle.play_raw(source2.convert_samples()).expect("TODO: panic message");

// The sound plays in a separate audio thread,
// so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(100));
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
//      Aynı kelimeyi farklı insanlar söyleyip mikrofonla kayıt mı etsin ?                                        //
//      İnsanlar farklı cümleler kursun, program ise onların seslerinin frekansını algılayıp kayıt mı etsin ?     //
// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //