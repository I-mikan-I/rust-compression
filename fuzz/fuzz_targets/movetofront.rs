#![no_main]
use libfuzzer_sys::fuzz_target;
use compression::Coder;
use compression::MoveToFront;

fuzz_target!(|data: &[u8]| {
    let output = MoveToFront::decode(MoveToFront::encode(data));
    assert_eq!(Vec::from(data), output);
});