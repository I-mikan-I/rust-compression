#![no_main]
use compression::Coder;
use compression::MoveToFront;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let output = MoveToFront::decode(MoveToFront::encode(data).unwrap()).unwrap();
    assert_eq!(Vec::from(data), output);
});
