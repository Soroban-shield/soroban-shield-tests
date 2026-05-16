use proptest::prelude::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::access_control;

proptest! {
    #[test]
    fn granted_role_is_queryable(_n in 1u32..50) {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        env.mock_all_auths();
        let role = symbol_short!("role");
        access_control::grant_role(&env, &role, &user, &admin);
        prop_assert!(access_control::has_role(&env, &role, &user));
    }
}
