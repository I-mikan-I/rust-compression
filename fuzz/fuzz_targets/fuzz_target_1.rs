#![no_main]
use libfuzzer_sys::fuzz_target;
use compression::Coder;
use compression::{Bwt,MoveToFront,Huffman};

fuzz_target!(|data: &[u8]| {
    let output = Bwt::decode(MoveToFront::decode(Huffman::decode(Huffman::encode(MoveToFront::encode(Bwt::encode(data))))));
    assert_eq!(Vec::from(data), output);
});