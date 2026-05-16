use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::{ownable, pausable};

#[test]
fn owner_pauses_contract() {
    let env = Env::default();
    let owner = Address::generate(&env);
    env.mock_all_auths();
    ownable::initialize(&env, &owner);
    pausable::pause(&env, &owner);
    assert!(pausable::is_paused(&env));
}
