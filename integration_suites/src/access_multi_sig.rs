use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::{access_control, multi_sig};

#[test]
fn role_gated_proposal() {
    let env = Env::default();
    let admin = Address::generate(&env);
    env.mock_all_auths();
    access_control::grant_role(&env, &symbol_short!("admin"), &admin, &admin);
    multi_sig::set_threshold(&env, 1);
    assert_eq!(
        multi_sig::create_proposal(&env, &admin, symbol_short!("x"), 1),
        1
    );
}
