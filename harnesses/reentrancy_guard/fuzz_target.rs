#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut depth = 0i32;
    for b in data {
        if b % 2 == 0 {
            depth += 1;
        } else {
            depth = depth.saturating_sub(1);
        }
    }
    assert!(depth >= 0);
});
