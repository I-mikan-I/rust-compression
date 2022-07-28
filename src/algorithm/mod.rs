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

impl<T, I, O> Coder<I, O> for &T
where
    T: Coder<I, O>,
    I: Copy,
    O: Copy,
{
    type Error = T::Error;

    fn encode(input: impl AsRef<[I]>) -> Result<Vec<O>, Self::Error> {
        T::encode(input)
    }

    fn decode(input: impl AsRef<[O]>) -> Result<Vec<I>, Self::Error> {
        T::decode(input)
    }

    fn encode_s(&self, input: impl AsRef<[I]>) -> Result<Vec<O>, Self::Error> {
        T::encode_s(self, input)
    }

    fn decode_s(&self, input: impl AsRef<[O]>) -> Result<Vec<I>, Self::Error> {
        T::decode_s(self, input)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    fn is_normal<T: Send + Sync + Debug + Sized + Unpin>() {}

    macro_rules! normal_battery {
        ( $( $t:ty ),* ) => {
            mod normal_typechecks {
                #[test]
                fn test_battery() {
                    $(super::is_normal::<$t>());*
                }
            }
        };
    }

    normal_battery!(
        crate::algorithm::bwt::Bwt,
        crate::algorithm::huffman::Huffman,
        crate::algorithm::movetofront::MoveToFront
    );
}
