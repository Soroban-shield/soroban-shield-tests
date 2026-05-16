#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let window = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let _ = window % 3600;
});
