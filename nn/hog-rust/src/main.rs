mod pixels;
mod processing;

#[macro_use] extern crate log;

use std::fs::File;
use std::{env};

fn main() {
    let args: Vec<_> = env::args().map(|arg| arg.clone()).collect();
    let file = File::open(args[1].clone()).expect("failed to open file");
    let pixels:Vec<Vec<u8>>;

    match pixels::get("jpeg", file) {
        Ok(res) => pixels = res,
        Err(e) => panic!("get_pixels: {:?}", e)
    }

    println!("{:?}", pixels);
}
