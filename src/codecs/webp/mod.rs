//! Decoding of WebP Images

pub use self::decoder::WebPDecoder;

mod decoder;
mod transform;
mod loop_filter;

mod lossless;
mod lossless_transform;
mod huffman;

pub mod vp8;
