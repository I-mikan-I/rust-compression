mod bwt;
mod huffman;
mod movetofront;
pub use bwt::*;
pub use huffman::*;
pub use movetofront::*;
pub trait Coder<I: Copy, O: Copy> {
    type Error;
    fn encode(input: impl AsRef<[I]>) -> Result<Vec<O>, Self::Error>;
    fn decode(input: impl AsRef<[O]>) -> Result<Vec<I>, Self::Error>;
    fn encode_s(&self, input: impl AsRef<[I]>) -> Result<Vec<O>, Self::Error> {
        Self::encode(input)
    }
    fn decode_s(&self, input: impl AsRef<[O]>) -> Result<Vec<I>, Self::Error> {
        Self::decode(input)
    }
}
