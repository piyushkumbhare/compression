#![allow(unused)]

use std::{
    fs::File,
    io::{Read, Write},
};
mod utils;
use utils::{
    bwt, compress, mtf, rle,
    tokens::{Token, Tokens},
};

fn main() {
    let mut buf = String::new();

    let path = "example.txt";

    let path_vec = path.split(".").collect::<Vec<_>>();
    let pathname = path_vec.get(0..path_vec.len() - 1).unwrap().concat();
    let mut infile = File::open(path).unwrap();
    infile.read_to_string(&mut buf).unwrap();

    let pipeline: Vec<&dyn Fn(&str) -> String> = vec![
        &bwt::encode,
        &mtf::encode,
        &bwt::encode,
        &rle::encode,
    ];

    let output = compress::compress_pipeline(&buf, &pipeline);

    let percent = (1.0 - output.len() as f32 / buf.len() as f32) * 100.0;
    println!(
        "Total compression: {} -> {} bytes ({}% compression)",
        buf.len(),
        output.len(),
        percent
    );

    let mut outfile = File::create(format!("{}.pkzip", pathname)).unwrap();
    outfile.write_all(&output.as_bytes());
}

/*
    plan for future:
    in order to test the efficiency of different orders of encoding/compression,
    i want to make a pipelining tool:

    essentially a Vec<Fn(&str) -> String> that something can iterate over.
    will take the output of one and pipe it into the input of the next

    This will allow me to customize any number of combinations easily via:

        let s = some_string();
        let pipeline: Vec<_> = vec![rle::encode, bwt::encode, mtf::encode, rle::encode, ...];
        let output = compress(&s, pipeline);

*/
