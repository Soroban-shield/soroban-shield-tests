use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::{access_control, rate_limiter};

#[test]
fn admin_bypasses_rate_limit_config() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    let ok = env.as_contract(&contract_id, || {
        access_control::grant_role(&env, &symbol_short!("admin"), &admin, &admin);
        rate_limiter::configure(&env, 100, 60);
        rate_limiter::check_and_record(&env, &admin, 0).is_ok()
    });
    assert!(ok);
}
