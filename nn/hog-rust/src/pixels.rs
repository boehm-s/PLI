extern crate jpeg_decoder as jpeg;

use std::fs::File;
use std::io::BufReader;
use matrix::Matrix;


pub struct GrayscaleImage {
    pub pixel_matrix: Matrix<u8>
}

pub struct RGBImage {
    pub pixel_matrix: Matrix<Vec<u8>>,
}

impl RGBImage {

    pub fn new(format:&str, file:File) -> Result<RGBImage, String> {
        match format {
            "jpeg" => Ok(RGBImage::from_jpeg(file)?),
            _ => Err(String::from("Image format not supported !"))
        }
    }


    pub fn from_jpeg(file:File) -> Result<RGBImage, String> {
        let mut decoder = jpeg::Decoder::new(BufReader::new(file));
        decoder.read_info().expect("Cannot read size !");
        let info = decoder.info().unwrap();
        match decoder.decode() {
            Ok(pixels) => Ok(RGBImage {
                pixel_matrix: Matrix::new(
                    info.width as usize,
                    info.height as usize,
                    pixels
                        .chunks(3)
                        .map(|x| { let mut y = x.to_vec(); y})
                        .collect::<Vec<_>>()
                )
            }),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn to_grayscale(&mut self) -> Result<GrayscaleImage, String> {
        /// luminosity method : 0.21 * r + 0.72 * g + 0.07 * b
    }
}
