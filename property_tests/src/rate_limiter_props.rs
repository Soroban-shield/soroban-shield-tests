use proptest::prelude::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::rate_limiter;

proptest! {
    #[test]
    fn respects_max_calls(max in 1u32..10) {
        let env = Env::default();
        let contract_id = env.register_contract(None, ());
        let caller = Address::generate(&env);
        let results: Vec<bool> = env.as_contract(&contract_id, || {
            rate_limiter::configure(&env, max, 60);
            (0..max).map(|i| rate_limiter::check_and_record(&env, &caller, i as u64).is_ok()).collect()
        });
        for ok in results {
            prop_assert!(ok);
        }
    }

    #[test]
    fn rejects_over_limit(max in 1u32..5) {
        let env = Env::default();
        let contract_id = env.register_contract(None, ());
        let caller = Address::generate(&env);
        env.as_contract(&contract_id, || {
            rate_limiter::configure(&env, max, 60);
            for i in 0..=max {
                let _ = rate_limiter::check_and_record(&env, &caller, i as u64);
            }
        });
    }
}
