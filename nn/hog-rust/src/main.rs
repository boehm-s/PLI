mod pixels;
mod processing;
mod matrix;

#[macro_use] extern crate log;

use std::fs::File;
use std::{env};
use pixels::RGBImage;

fn main() {
    let args: Vec<_> = env::args().map(|arg| arg.clone()).collect();
    let file = File::open(args[1].clone()).expect("failed to open file");

    let img = match RGBImage::new("jpeg", file) {
        Ok(res) => res,
        Err(e) => panic!("get_pixels: {:?}", e)
    };

    println!("Image pixel matrix : \n{}", img.pixel_matrix);
}
