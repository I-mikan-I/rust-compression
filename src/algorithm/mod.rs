mod bwt;
mod huffman;
mod movetofront;
pub use bwt::*;
pub use huffman::*;
pub use movetofront::*;
pub trait Coder<I: Copy, O: Copy> {
    fn encode(input: impl AsRef<[I]>) -> Vec<O>;
    fn decode(input: impl AsRef<[O]>) -> Vec<I>;
    fn encode_s(&self, input: impl AsRef<[I]>) -> Vec<O> {
        Self::encode(input)
    }
    fn decode_s(&self, input: impl AsRef<[O]>) -> Vec<I> {
        Self::decode(input)
    }
}
