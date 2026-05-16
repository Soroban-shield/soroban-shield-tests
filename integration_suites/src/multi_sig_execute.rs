use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::multi_sig;

#[test]
fn execute_meets_threshold() {
    let env = Env::default();
    multi_sig::execute(&env, 1, 2, 2);
}
