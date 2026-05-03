# soroban-shield-tests

> Dedicated fuzz harnesses, property-based tests, and integration suites for all `soroban-shield-contracts` modules.

[![Stellar Wave](https://img.shields.io/badge/Stellar%20Wave-Wave%205-blue?style=flat-square)](https://www.drips.network/wave/stellar)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat-square)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](LICENSE)
[![CI](https://img.shields.io/badge/CI-GitHub%20Actions-green?style=flat-square)](.github/workflows/ci.yml)

---

## Overview

`soroban-shield-tests` is a standalone repository containing all testing infrastructure for the Soroban Shield contract library. Separating tests from contracts keeps the core crate lean for WASM compilation while allowing the test suite to use heavier tooling (`cargo-fuzz`, `proptest`, simulation environments) without affecting production build size.

This repo covers three testing layers:

| Layer | Tool | Purpose |
|---|---|---|
| **Fuzz** | `cargo-fuzz` | Find unexpected panics and edge cases via mutation |
| **Property** | `proptest` | Verify invariants hold across arbitrary inputs |
| **Integration** | `soroban-sdk/testutils` | Multi-contract interaction and composability tests |

---

## File Structure

```
soroban-shield-tests/
│
├── Cargo.toml
├── README.md                               # This file
├── CONTRIBUTING.md
├── LICENSE
├── CODEOWNERS
├── .gitignore
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                          # Run all test layers on every PR
│   │   └── fuzz_schedule.yml              # Nightly scheduled fuzz runs (6 hrs)
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       └── stellar_wave_task.md
│
├── harnesses/                             # cargo-fuzz fuzz targets (one per module)
│   ├── Cargo.toml
│   ├── access_control/
│   │   └── fuzz_target.rs                 # Fuzzes role grant/revoke sequences
│   ├── ownable/
│   │   └── fuzz_target.rs                 # Fuzzes ownership transfer sequences
│   ├── pausable/
│   │   └── fuzz_target.rs                 # Fuzzes pause state transitions
│   ├── reentrancy_guard/
│   │   └── fuzz_target.rs                 # Fuzzes nested guard enter/exit
│   ├── rate_limiter/
│   │   └── fuzz_target.rs                 # Fuzzes sliding window with random timestamps
│   └── multi_sig/
│       └── fuzz_target.rs                 # Fuzzes proposal lifecycle sequences
│
├── property_tests/                        # proptest invariant suites
│   ├── mod.rs
│   ├── ownable_props.rs                   # Owner is always defined after init
│   ├── access_control_props.rs            # Role admin cannot exceed its own permissions
│   ├── pausable_props.rs                  # Paused state is binary; no partial pause
│   ├── rate_limiter_props.rs              # Count never exceeds max_calls in window
│   └── multi_sig_props.rs                # Quorum threshold is always respected
│
├── integration_suites/                   # Multi-contract interaction tests
│   ├── mod.rs
│   ├── ownable_pausable.rs               # Ownable-gated pause/unpause
│   ├── access_multi_sig.rs               # Role-gated multi-sig proposals
│   ├── upgradeable_proxy.rs              # Upgrade flow with state migration
│   ├── rate_limiter_access.rs            # Rate limit bypass via role elevation
│   └── full_composition.rs              # All modules composed: real-world scenario
│
└── fixtures/
    ├── mod.rs
    ├── addresses.rs                      # Deterministic test address generators
    ├── environments.rs                   # Soroban Env builders for common setups
    └── contracts.rs                      # Pre-deployed contract instances for tests
```

---

## Running Tests

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-fuzz
cargo install cargo-fuzz

# Install the nightly toolchain (required for fuzzing)
rustup toolchain install nightly
```

### Property and Integration Tests

```bash
git clone https://github.com/soroban-shield/soroban-shield-tests
cd soroban-shield-tests

# Run all property tests and integration suites
cargo test --features testutils
```

### Fuzz Testing

```bash
# Fuzz the AccessControl module for 60 seconds
cargo +nightly fuzz run access_control -- -max_total_time=60

# Fuzz the MultiSig module
cargo +nightly fuzz run multi_sig -- -max_total_time=60

# List all available fuzz targets
cargo +nightly fuzz list
```

### Run Everything (CI-style)

```bash
./scripts/run_all_tests.sh
```

---

## Writing New Tests

### Adding a Fuzz Harness

Create a new file under `harnesses/<module>/fuzz_target.rs`. Use the `fuzz_target!` macro:

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Drive your contract with arbitrary input derived from `data`
});
```

Register it in `harnesses/Cargo.toml`.

### Adding a Property Test

Add to the corresponding file in `property_tests/`:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn owner_is_always_set_after_init(addr in any::<[u8; 32]>()) {
        // setup env, init contract, assert invariant
    }
}
```

### Adding an Integration Test

Add to `integration_suites/`. Each test should:
1. Create a fresh `soroban_sdk::Env` (use `fixtures::environments`)
2. Deploy required contracts
3. Execute the scenario
4. Assert the expected state

---

## Stellar Wave — Open Issues

Issues labeled `Stellar Wave` in this repo are available for contributors during the Stellar Wave sprint.

Browse: [github.com/soroban-shield/soroban-shield-tests/issues](https://github.com/soroban-shield/soroban-shield-tests/issues?q=label%3A%22Stellar+Wave%22)

**Points:** Trivial = 100 pts | Medium = 150 pts | High = 200 pts

Full rules: [docs.drips.network/wave/terms-and-rules](https://docs.drips.network/wave/terms-and-rules)

---

## License

MIT — see [LICENSE](LICENSE)
