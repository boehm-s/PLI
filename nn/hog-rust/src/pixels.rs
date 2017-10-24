extern crate jpeg_decoder as jpeg;

use std::fs::File;
use std::io::BufReader;
use matrix::Matrix;



pub struct PixelsImage {
    pub rgb_matrix: Matrix<Vec<u8>>,
    pub grayscale_matrix: Matrix<u8>
}

impl PixelsImage {

    pub fn new() -> PixelsImage {
        PixelsImage {
            rgb_matrix: Matrix::new(0, 0, vec![vec![]]),
            grayscale_matrix: Matrix::new(0, 0, vec![])
        }
    }

    pub fn from_file(format:&str, file:File) -> Result<PixelsImage, String> {
        match format {
            "jpeg" => Ok(PixelsImage::from_jpeg(file)?),
            _ => Err(String::from("Image format not supported !"))
        }

    }

    pub fn from_jpeg(file:File) -> Result<PixelsImage, String> {
        let mut decoder = jpeg::Decoder::new(BufReader::new(file));
        decoder.read_info().expect("Cannot read size !");
        let info = decoder.info().unwrap();
        match decoder.decode() {
            Ok(pixels) => Ok(PixelsImage {
                rgb_matrix: Matrix::new(
                    info.width as usize,
                    info.height as usize,
                    pixels
                        .chunks(3)
                        .map(|x| { let y = x.to_vec(); y})
                        .collect::<Vec<_>>()
                ),
                grayscale_matrix: Matrix::new(
                    info.width as usize,
                    info.height as usize,
                    pixels
                        .chunks(3)
                        .map(|x| rgb_to_gray(x.to_vec()))
                        .collect::<Vec<_>>()
                )
            }),
            Err(e) => Err(e.to_string())
        }
    }
}

fn rgb_to_gray(rgb:Vec<u8>) -> u8 {
    (0.21 * rgb[0] as f32 + 0.72 * rgb[1] as f32 + 0.07 * rgb[2] as f32) as u8
}
