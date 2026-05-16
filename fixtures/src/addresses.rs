use soroban_sdk::{testutils::Address as _, Address, Env};

pub fn admin(env: &Env) -> Address {
    Address::generate(env)
}

pub fn user(env: &Env) -> Address {
    Address::generate(env)
}
