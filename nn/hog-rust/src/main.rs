mod pixels;
mod processing;

#[macro_use] extern crate log;

use std::fs::File;
use std::{env};

fn main() {
    let args: Vec<_> = env::args().map(|arg| arg.clone()).collect();
    let file = File::open(args[1].clone()).expect("failed to open file");

    let img = match pixels::get("jpeg", file) {
        Ok(res) => res,
        Err(e) => panic!("get_pixels: {:?}", e)
    };

    println!("PIXELS: {:?}\nMETADATA: {:?}\n RAW DATA : {:?}", img.pixel_data, img.metadata, img.raw_data);
}
