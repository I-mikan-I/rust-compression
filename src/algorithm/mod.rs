mod bwt;
mod huffman;
mod movetofront;
pub use bwt::*;
pub use huffman::*;
pub use movetofront::*;
pub trait Coder<I: Copy, O: Copy> {
    fn encode(input: &[I]) -> Vec<O>;
    fn decode(input: &[O]) -> Vec<I>;
    fn encode_s(&self, input: &[I]) -> Vec<O> {
        Self::encode(input)
    }
    fn decode_s(&self, input: &[O]) -> Vec<I> {
        Self::decode(input)
    }
}
