#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    for byte in data.iter().take(32) {
        let _ = byte % 2;
    }
});
