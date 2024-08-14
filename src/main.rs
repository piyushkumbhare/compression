#![allow(unused)]

use std::{
    fs::File,
    io::{Read, Write},
};
mod utils;
use compress::Encoders;
use utils::{
    bwt, compress, mtf, rle,
    tokens::{Token, Tokens},
};

/*
    In case anyone starts reading the code from here, I suggest you read each of
    the following files for a detailed explanation of each module and its purpose
    in this project:
        - compress.rs
        - bwt.rs
        - rle.rs
        - mtf.rs
        - utils.rs

    If you're still curious on what goes on in this main function, all that's happening is:
    
        1. I read in a file with a given path and store it in a String variable
        2. I use the pipeline!() macro to create the compress & decompress pipelines (see compress.rs)
        3. I pass the String into this pipeline and track compression rate via .len()
        4. I decompress the string and assert that the decompressed string matches the original string.

    Really nothing special...
*/

fn main() {
    let mut buf = String::new();

    let path = "example.txt";

    let path_vec = path.split(".").collect::<Vec<_>>();
    let pathname = path_vec.get(0..path_vec.len() - 1).unwrap().concat();
    let mut infile = File::open(path).unwrap();
    infile.read_to_string(&mut buf).unwrap();

    let pipeline = vec![Encoders::BWT, Encoders::RLE];
    let (compress, decompress) = pipeline!(BWT, MTF);

    let output = compress::execute_pipeline(&buf, &compress);

    let percent = (1.0 - output.len() as f32 / buf.len() as f32) * 100.0;
    println!(
        "Total compression: {} -> {} bytes ({}% compression)",
        buf.len(),
        output.len(),
        percent
    );

    let mut outfile = File::create(format!("{}.pkzip", pathname)).unwrap();
    outfile.write_all(&output.as_bytes());

    let decoded_string = compress::execute_pipeline(&output, &decompress);

    let mut decoded_file = File::create(format!("decoded-{path}")).unwrap();
    decoded_file.write_all(decoded_string.as_bytes());

    if decoded_string == buf {
        println!("Encoded file decodes back to the original file, good job!");
    } else {
        println!("Decoded file & original file differ...");
    }
}

