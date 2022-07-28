#![no_main]
use libfuzzer_sys::fuzz_target;
use compression::Coder;
use compression::Huffman;

fuzz_target!(|data: &[u8]| {
    let output = Huffman::decode(Huffman::encode(data));
    assert_eq!(Vec::from(data), output);
});