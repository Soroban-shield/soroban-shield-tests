use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::ownable;

#[test]
fn renounce_clears_owner() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let owner = Address::generate(&env);
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        ownable::initialize(&env, &owner);
        ownable::renounce_ownership(&env);
    });
}
