use soroban_sdk::Env;
use soroban_shield_contracts::contracts::multi_sig;

#[test]
fn execute_meets_threshold() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    env.as_contract(&contract_id, || {
        multi_sig::execute(&env, 1, 2, 2);
    });
}
