#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut paused = false;
    for b in data {
        paused = if b % 2 == 0 { true } else { false };
    }
    let _ = paused;
});
