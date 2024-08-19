use crate::utils::encoder_trait::Encoder;

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

pub struct Pipeline(Vec<Box<dyn Encoder + 'static>>);

impl Pipeline {
    pub fn new(encoders: Vec<Box<dyn Encoder + 'static>>) -> Self {
        Self(encoders)
    }

    pub fn compress(&self, data: &str) -> String {
        let mut data: String = data.to_string();
        for stage in self.0.iter() {
            data = stage.encode(&data);
        }
        data
    }

    pub fn decompress(&self, data: &str) -> String {
        let mut data: String = data.to_string();
        for stage in self.0.iter().rev() {
            data = stage.decode(&data);
        }
        data
    }
}

/// An Enum that holds all encoders. This is used for ease of use with the pipeline! macro.
pub enum DefinedEncoders {
    /// The Burrows-Wheeler Transform
    BWT,
    /// Move-To-Front Encoding
    MTF,
    /// Run-Length Encoding
    RLE,
}

#[macro_export]
macro_rules! create_pipeline {
    ($( $i:ident ),*) => {
        {
            use crate::utils::compress::DefinedEncoders::*;
            use crate::utils::encoders::{bwt, mtf, rle};

            let mut v: Vec<Box<dyn Encoder>> = Vec::new();
            $(
                let encoder: Box<dyn Encoder> = match $i {
                    BWT => Box::new(bwt::BWT{}),
                    RLE => Box::new(rle::RLE{}),
                    MTF => Box::new(mtf::MTF{}),
                };
                v.push(encoder);
            )*
            compress::Pipeline::new(v)
        }
    };
}
