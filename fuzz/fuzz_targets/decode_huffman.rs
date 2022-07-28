#![no_main]
use compression::Coder;
use compression::Huffman;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Huffman::decode(data);
});
