#![no_main]
use compression::Bwt;
use compression::Coder;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Bwt::decode(data);
});
