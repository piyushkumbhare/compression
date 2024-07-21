use std::{fs::File, io::{Read, Write}};
mod utils;
use utils::bwt::BWT;
fn main() {
    let mut buf = String::new();
    
    let path = "example.txt"; 
    
    let mut infile = File::open(path).unwrap();
    infile.read_to_string(&mut buf).unwrap();

    println!("Encoding string");
    let bwt = BWT::encode(&buf);
    println!("Encoding finished");

    let mut outfile = File::create(format!("bwt-{}", path)).unwrap();
    outfile.write_all(&bwt.as_bytes()).unwrap();

    let decoded = BWT::decode(&bwt);
    println!("Decoded string: {}", &decoded);
}
