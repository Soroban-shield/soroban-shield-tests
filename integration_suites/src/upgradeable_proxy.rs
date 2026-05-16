use soroban_sdk::{BytesN, Env};
use soroban_shield_contracts::contracts::upgradeable;

#[test]
fn upgrade_after_init() {
    let env = Env::default();
    let h1 = BytesN::from_array(&env, &[1u8; 32]);
    let h2 = BytesN::from_array(&env, &[2u8; 32]);
    upgradeable::set_implementation(&env, &h1);
    upgradeable::upgrade(&env, &h2);
    assert_eq!(upgradeable::current_implementation(&env), h2);
}
