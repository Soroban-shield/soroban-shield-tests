use soroban_sdk::{BytesN, Env};
use soroban_shield_contracts::contracts::upgradeable;

#[test]
fn upgrade_after_init() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let h1 = BytesN::from_array(&env, &[1u8; 32]);
    let h2 = BytesN::from_array(&env, &[2u8; 32]);
    let current = env.as_contract(&contract_id, || {
        upgradeable::set_implementation(&env, &h1);
        upgradeable::upgrade(&env, &h2);
        upgradeable::current_implementation(&env)
    });
    assert_eq!(current, h2);
}
