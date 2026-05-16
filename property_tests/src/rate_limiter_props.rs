use proptest::prelude::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::rate_limiter;

proptest! {
    #[test]
    fn respects_max_calls(max in 1u32..10) {
        let env = Env::default();
        let caller = Address::generate(&env);
        rate_limiter::configure(&env, max, 60);
        for i in 0..max {
            let ok = rate_limiter::check_and_record(&env, &caller, i as u64).is_ok();
            prop_assert!(ok);
        }
    }

    #[test]
    fn rejects_over_limit(max in 1u32..5) {
        let env = Env::default();
        let caller = Address::generate(&env);
        rate_limiter::configure(&env, max, 60);
        for i in 0..=max {
            let _ = rate_limiter::check_and_record(&env, &caller, i as u64);
        }
    }
}
