use crate::utils::{rle, bwt, mtf};

/*
    This is the main tool used to invoke the Compression Tool. 

    The pipeline!() macro takes any number of Encoders Enums and spits out
    2 Vecs of functions. The first is the forward (encoder) Vec, and the second
    is the backward (decoder) Vec.

    These 2 Vecs can be passed into the execute_pipeline() function, which takes
    a starting data input along with a Vec of functions. It will then attempt to
    input the data into the first function, then use its output as an input into
    the next, etc. 

    For example, the following code creates a Compression Pipeline where the order is
    BWT -> RLE -> MTF -> RLE:

    let (encoder, decoder) = pipeline!(BWT, RLE, MTF, RLE);
    let compressed = execute_pipeline(&some_data, &encoder);
    let decompressed = execute_pipeline(&some_data, &decoder);
    
*/
pub enum Encoders {
    BWT,
    RLE,
    MTF,
    // HUFF,
    // other encoding schemes
    // ...
}

pub fn execute_pipeline(data: &str, pipeline: &Vec<&dyn Fn(&str) -> String>) -> String {
    let mut data: String = data.to_string();
    for &stage in pipeline {
        data = stage(&data);
    }
    data
}

#[macro_export]
macro_rules! pipeline {
    ( $( $encoder:ident ),* ) => {
        {
            let compress_pipeline: Vec<&dyn Fn(&str) -> String> = vec![
                $(
                    match Encoders::$encoder {
                        Encoders::BWT => &bwt::encode,
                        Encoders::RLE => &rle::encode,
                        Encoders::MTF => &mtf::encode,
                        // Encoders::HUFF => &huff::encode,
                    },
                )*
            ];

            let decompress_pipeline: Vec<&dyn Fn(&str) -> String> = vec![
                $(
                    match Encoders::$encoder {
                        Encoders::BWT => &bwt::decode,
                        Encoders::RLE => &rle::decode,
                        Encoders::MTF => &mtf::decode,
                        // Encoders::HUFF => &huff::decode,
                    },
                )*
            ];
            
            let decompress_pipeline: Vec<&dyn Fn(&str) -> String> = decompress_pipeline.into_iter().rev().collect();
            (compress_pipeline, decompress_pipeline)
        }
    };
}
