use crate::Coder;
use std::cmp::min;
use std::error::Error;

const DECODE_ERROR: &str = "input file is not properly encoded";

#[derive(Clone, Debug)]
pub struct Bwt {
    block_pow: usize,
}

impl Bwt {
    pub const fn new(block_pow: usize) -> Self {
        if block_pow > 32 {
            panic!("Only up to 4GiB large blocks supported!")
        }
        Self { block_pow }
    }
}

impl Coder<u8, u8> for Bwt {
    type Error = Box<dyn Error + Send + Sync + 'static>;
    fn encode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        Bwt::new(20).encode_s(input)
    }
    fn decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        let input = input.as_ref();
        if input.len() < 4 {
            return Err(DECODE_ERROR.into());
        }
        let mut output = Vec::with_capacity(input.len());
        let block_len: u32 = u32::from_le_bytes((&input[0..4]).try_into()?);
        let input = &input[4..];
        let mut index = 0;
        while index + 4 < input.len() {
            let block_len: usize = min(block_len as usize, input.len() - index - 4);
            let row = u32::from_le_bytes((&input[index..][..4]).try_into()?);
            let last_column = &input[index + 4..][..block_len];
            let mut first_column: Vec<_> = last_column.iter().copied().enumerate().collect();
            index += block_len + 4;
            first_column.sort_by_key(|(_, b)| *b);
            let mut i = row as usize;
            for _ in 0..first_column.len() {
                let (next, val) = *first_column
                    .get(i)
                    .ok_or_else(|| Self::Error::from(DECODE_ERROR))?;
                output.push(val);
                i = next;
            }
        }
        Ok(output)
    }
    fn encode_s(&self, input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        let input = input.as_ref();
        let mut output = Vec::new();
        let block_size: u32 = 1 << self.block_pow;
        let table: Vec<u32> = (0..block_size).into_iter().collect();
        let mut index = 0;
        output.extend((min(block_size as usize, input.len()) as u32).to_le_bytes());

        while index < input.len() {
            let block_size = min(block_size as usize, input.len() - index);
            let current_block = &input[index..][..block_size];
            index += block_size;
            let mut current_table: Vec<u32> = table[..block_size].into();

            current_table.sort_by(|&i1, &i2| {
                let (i1, i2) = (i1 as usize, i2 as usize);
                current_block[i1..]
                    .iter()
                    .chain(current_block[..i1].iter())
                    .cmp(current_block[i2..].iter().chain(current_block[..i2].iter()))
            });

            let mut original_position: u32 = 0;
            let mut current_output: Vec<u8> = current_table
                .into_iter()
                .enumerate()
                .map(|(e, index)| {
                    if index == 0 {
                        original_position = e as u32;
                    }
                    current_block[(index as isize - 1).rem_euclid(block_size as isize) as usize]
                })
                .collect();

            output.extend(original_position.to_le_bytes());
            output.append(&mut current_output);
        }
        Ok(output)
    }
}
#[cfg(test)]
mod tests {
    use crate::algorithm::bwt::Bwt;
    use crate::Coder;

    #[test]
    fn transform() {
        let input = [3_u8, 8, 8, 3, 2, 1];
        let output = Bwt::decode(&Bwt::encode(&input).unwrap()).unwrap();
        assert_eq!(Vec::from(input), output)
    }

    #[test]
    fn transform_2() {
        let input = &[46_u8, 46];
        let output = Bwt::decode(Bwt::encode(input).unwrap()).unwrap();
        assert_eq!(Vec::from(&input[..]), output)
    }

    #[test]
    fn transform_large() {
        let b = Bwt::new(10);
        let input: Vec<_> = (0..(1 << 10) + 2347)
            .into_iter()
            .map(|_| rand::random::<u8>())
            .collect();
        let output = b.decode_s(&b.encode_s(&input).unwrap()).unwrap();
        assert_eq!(input, output)
    }

    #[test]
    fn decode_unencoded_1() {
        let input = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xa];
        let _ = Bwt::decode(input);
    }

    #[test]
    fn empty() {
        let _ = Bwt::decode(Bwt::encode([]).unwrap()).unwrap();
    }
}
