use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::ownable;

#[test]
fn renounce_clears_owner() {
    let env = Env::default();
    let owner = Address::generate(&env);
    env.mock_all_auths();
    ownable::initialize(&env, &owner);
    ownable::renounce_ownership(&env);
}
