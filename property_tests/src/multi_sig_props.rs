use proptest::prelude::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::multi_sig;

proptest! {
    #[test]
    fn proposal_ids_increment(threshold in 1u32..5) {
        let env = Env::default();
        let contract_id = env.register_contract_wasm(None, &[]);
        let creator = Address::generate(&env);
        env.mock_all_auths();
        let (id1, id2) = env.as_contract(&contract_id, || {
            multi_sig::set_threshold(&env, threshold);
            let id1 = multi_sig::create_proposal(&env, &creator, symbol_short!("a"), 100);
            let id2 = multi_sig::create_proposal(&env, &creator, symbol_short!("b"), 200);
            (id1, id2)
        });
        prop_assert_eq!(id2, id1 + 1);
    }
}
