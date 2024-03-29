#![no_main]
use compression::Coder;
use compression::Huffman;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let output = Huffman::decode(Huffman::encode(data).unwrap()).unwrap();
    assert_eq!(Vec::from(data), output);
});
