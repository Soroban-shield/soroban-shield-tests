#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut proposals = 0u64;
    for b in data {
        if b % 5 == 0 {
            proposals += 1;
        }
    }
    let _ = proposals;
});
