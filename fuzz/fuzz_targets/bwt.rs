#![no_main]
use libfuzzer_sys::fuzz_target;
use compression::Coder;
use compression::Bwt;

const B: Bwt = Bwt::new(8);

fuzz_target!(|data: &[u8]| {
    let output = B.decode_s(B.encode_s(data));
    assert_eq!(Vec::from(data), output);
});