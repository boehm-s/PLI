extern crate jpeg_decoder as jpeg;

use std::fs::File;
use std::io::BufReader;
use matrix::Matrix;

#[derive(Debug)]
pub struct ImageSize {
    pub width: u16,
    pub height: u16
}

pub struct RgbaImage {
    pub raw_data: Vec<u8>,
    pub pixel_matrix: Matrix<Vec<u8>>,
    pub size: ImageSize
}

fn get_from_jpeg(file:File) -> Result<RgbaImage, String> {
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    decoder.read_info().expect("Cannot read size !");
    let info = decoder.info().unwrap();
    match decoder.decode() {
        Ok(pixels) => Ok(RgbaImage {
            raw_data: pixels
                .chunks(3)
                .map(|x| { let mut y = x.to_vec(); y.push(1); y})
                .fold(Vec::new(), |mut acc, mut x| { acc.append(&mut x); acc }),
            pixel_matrix: Matrix::new(
                info.width as usize,
                info.height as usize,
                pixels
                    .chunks(3)
                    .map(|x| { let mut y = x.to_vec(); y.push(1); y})
                    .collect::<Vec<_>>()
            ),
            size: ImageSize {
                width: info.width,
                height: info.height
            }
        }),
        Err(e) => Err(e.to_string())
    }
}

pub fn get(format:&str, file:File) -> Result<RgbaImage, String> {
    match format {
        "jpeg" => Ok(get_from_jpeg(file)?),
        _ => Err(String::from("Image format not supported !"))
    }
}
