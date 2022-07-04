mod huffman;
pub use huffman::*;
pub trait Coder<I: Copy, O: Copy> {
    fn encode(input: &[I]) -> Vec<O>;
    fn decode(input: &[O]) -> Vec<I>;
}
