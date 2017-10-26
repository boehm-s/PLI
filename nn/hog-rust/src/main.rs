mod pixels;
mod processing;
mod matrix;

// #[macro_use] extern crate log;

use std::fs::File;
use std::{env};
use pixels::PixelsImage;

fn main() {
    let args: Vec<_> = env::args().map(|arg| arg.clone()).collect();
    let file = File::open(args[1].clone()).expect("failed to open file");

    let img = match PixelsImage::from_file("jpeg", file) {
        Ok(res) => res,
        Err(e) => panic!("get_pixels: {:?}", e)
    };

    println!("grayscale pixel matrix : \n");
    matrix::print_matrix_int(img.grayscale_matrix);

    println!("color pixel matrix : \n");
    matrix::print_matrix_vec(img.rgb_matrix);

}
