use proptest::prelude::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::pausable;

proptest! {
    #[test]
    fn pause_is_binary(toggles in prop::collection::vec(any::<bool>(), 1..20)) {
        let env = Env::default();
        let admin = Address::generate(&env);
        env.mock_all_auths();
        for on in toggles {
            if on { pausable::pause(&env, &admin); } else { pausable::unpause(&env, &admin); }
            prop_assert_eq!(pausable::is_paused(&env), on);
        }
    }
}
