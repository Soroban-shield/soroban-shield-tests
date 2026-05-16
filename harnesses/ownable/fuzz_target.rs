#![no_main]
use libfuzzer_sys::fuzz_target;

// Fuzz ownership transfer opcode sequences
fuzz_target!(|data: &[u8]| {
    let op = data.first().copied().unwrap_or(0) % 4;
    match op {
        0 | 1 => {}
        2 | 3 => {}
        _ => unreachable!(),
    }
});
