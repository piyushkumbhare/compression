pub fn compress(data: &str) -> Vec<u8> {
    todo!();
    // data = RLE(data)
    // data = BWT(data)
    // data = MTF(data)
    // data = RLE(data)
    // data = Huffman(data)
    // idk we'll go from there
}

pub fn compress_pipeline(data: &str, pipeline: &Vec<&dyn Fn(&str) -> String>) -> String {
    let mut data: String = data.to_string();
    for &stage in pipeline {
        data = stage(&data);
    }
    data
}
