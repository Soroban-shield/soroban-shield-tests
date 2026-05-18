use proptest::prelude::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::ownable;

proptest! {
    #[test]
    fn owner_is_set_after_init(_seed in 0u32..1000) {
        let env = Env::default();
        let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
        let owner = Address::generate(&env);
        env.mock_all_auths();
        let result = env.as_contract(&contract_id, || {
            ownable::initialize(&env, &owner);
            ownable::owner(&env)
        });
        prop_assert_eq!(result, owner);
    }

    #[test]
    fn transfer_then_accept(_ in 0u32..10) {
        let env = Env::default();
        let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
        let o1 = Address::generate(&env);
        let o2 = Address::generate(&env);
        env.mock_all_auths();
        let result = env.as_contract(&contract_id, || {
            ownable::initialize(&env, &o1);
            ownable::transfer_ownership(&env, &o2);
            ownable::accept_ownership(&env);
            ownable::owner(&env)
        });
        prop_assert_eq!(result, o2);
    }
}
