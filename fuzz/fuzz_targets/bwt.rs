#![no_main]
use compression::Bwt;
use compression::Coder;
use libfuzzer_sys::fuzz_target;

const B: Bwt = Bwt::new(8);

fuzz_target!(|data: &[u8]| {
    let output = B.decode_s(B.encode_s(data).unwrap()).unwrap();
    assert_eq!(Vec::from(data), output);
});
