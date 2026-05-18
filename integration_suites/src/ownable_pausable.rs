use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::{ownable, pausable};

#[test]
fn owner_pauses_contract() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let owner = Address::generate(&env);
    env.mock_all_auths();
    let paused = env.as_contract(&contract_id, || {
        ownable::initialize(&env, &owner);
        pausable::pause(&env, &owner);
        pausable::is_paused(&env)
    });
    assert!(paused);
}
