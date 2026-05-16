use proptest::prelude::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::ownable;

proptest! {
    #[test]
    fn owner_is_set_after_init(_seed in 0u32..1000) {
        let env = Env::default();
        let owner = Address::generate(&env);
        env.mock_all_auths();
        ownable::initialize(&env, &owner);
        prop_assert_eq!(ownable::owner(&env), owner);
    }
}
