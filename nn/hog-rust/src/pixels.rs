extern crate jpeg_decoder as jpeg;

use std::fs::File;
use std::io::BufReader;

fn get_from_jpeg(file:File) -> Result<Vec<Vec<u8>>, String> {
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    match decoder.decode() {
        Ok(pixels) => Ok(pixels.chunks(3).map(|x| {let mut y = x.to_vec(); y.insert(3,1); y}).collect::<Vec<_>>()),
        Err(e) => Err(e.to_string())
    }
}

pub fn get(format:&str, file:File) -> Result<Vec<Vec<u8>>, String> {
    match format {
        "jpeg" => Ok(get_from_jpeg(file)?),
        _ => Err(String::from("Image format not supported !"))
    }
}
