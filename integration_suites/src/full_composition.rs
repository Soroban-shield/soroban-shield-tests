use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::{ownable, pausable, reentrancy_guard};

#[test]
fn full_stack_guarded_flow() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let owner = Address::generate(&env);
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        ownable::initialize(&env, &owner);
        reentrancy_guard::non_reentrant(&env, |e| pausable::when_not_paused(e));
    });
}
