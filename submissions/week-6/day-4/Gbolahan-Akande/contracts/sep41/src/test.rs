#[cfg(test)]
mod tests {
    use crate::sep41;

    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn create_token_contract(env: &Env) -> (Address, sep41::Sep41Client) {
        let contract_id = env.register_contract(None, sep41::Sep41);
        let client = sep41::Sep41Client::new(env, &contract_id);
        (contract_id, client)
    }

    fn set_balance(env: &Env, address: &Address, amount: i128) {
        env.storage().persistent().set(address, &amount);
    }

    #[test]
    fn test_token_metadata() {
        let env = Env::default();
        let (_contract_id, client) = create_token_contract(&env);

        assert_eq!(client.decimals(), 18);
        assert_eq!(client.name(), String::from_str(&env, "Age Devs"));
        assert_eq!(client.symbol(), String::from_str(&env, "AGEDEVS"));
    }

    #[test]
    fn test_balance() {
        let env = Env::default();
        let (_contract_id, client) = create_token_contract(&env);
        let user = Address::generate(&env);

        assert_eq!(client.balance(&user), 0);

        set_balance(&env, &user, 1000);
        assert_eq!(client.balance(&user), 1000);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let (_contract_id, client) = create_token_contract(&env);
        
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        set_balance(&env, &alice, 1000);

        client.transfer(&alice, &bob, &300);

        assert_eq!(client.balance(&alice), 700);
        assert_eq!(client.balance(&bob), 300);
    }

    #[test]
    fn test_allowance_and_approve() {
        let env = Env::default();
        env.mock_all_auths();
        let (_contract_id, client) = create_token_contract(&env);
        
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);

        assert_eq!(client.allowance(&owner, &spender), 0);

        let future_ledger = env.ledger().sequence() + 100;
        client.approve(&owner, &spender, &500, &future_ledger);

        assert_eq!(client.allowance(&owner, &spender), 500);
    }

    #[test]
    fn test_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();
        let (_contract_id, client) = create_token_contract(&env);
        
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);

        set_balance(&env, &owner, 1000);
        let future_ledger = env.ledger().sequence() + 100;
        client.approve(&owner, &spender, &500, &future_ledger);

        client.transfer_from(&spender, &owner, &recipient, &200);

        assert_eq!(client.balance(&owner), 800);
        assert_eq!(client.balance(&recipient), 200);
        
        assert_eq!(client.allowance(&owner, &spender), 300);
    }

    #[test]
    fn test_burn() {
        let env = Env::default();
        env.mock_all_auths();
        let (_contract_id, client) = create_token_contract(&env);
        
        let user = Address::generate(&env);

        set_balance(&env, &user, 1000);

        client.burn(&user, &300);

        assert_eq!(client.balance(&user), 700);
    }

    #[test]
    fn test_burn_from() {
        let env = Env::default();
        env.mock_all_auths();
        let (_contract_id, client) = create_token_contract(&env);
        
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);

        set_balance(&env, &owner, 1000);
        let future_ledger = env.ledger().sequence() + 100;
        client.approve(&owner, &spender, &500, &future_ledger);

        client.burn_from(&spender, &owner, &200);

        assert_eq!(client.balance(&owner), 800);
        
        assert_eq!(client.allowance(&owner, &spender), 300);
    }

    #[test]
    fn test_self_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let (_contract_id, client) = create_token_contract(&env);
        
        let alice = Address::generate(&env);

        set_balance(&env, &alice, 1000);

        client.transfer(&alice, &alice, &100);

        assert_eq!(client.balance(&alice), 1000);
    }
}
