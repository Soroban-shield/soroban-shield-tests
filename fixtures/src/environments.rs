use soroban_sdk::Env;

pub fn default_env() -> Env {
    Env::default()
}

pub fn env_with_auth() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}
