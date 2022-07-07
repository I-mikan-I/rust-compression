mod huffman;
mod movetofront;
pub use huffman::*;
pub use movetofront::*;
pub trait Coder<I: Copy, O: Copy> {
    fn encode(input: &[I]) -> Vec<O>;
    fn decode(input: &[O]) -> Vec<I>;
}
