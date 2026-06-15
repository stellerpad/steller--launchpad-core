#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, String};

#[derive(Clone)]
#[contracttype]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub total_supply: i128,
    pub admin: Address,
    pub mintable: bool,
    pub burnable: bool,
    pub paused: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    Balance(Address),
    Allowance(Address, Address),
    TotalSupply,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
        total_supply: i128,
        mintable: bool,
        burnable: bool,
    ) {
        if env.storage().instance().has(&DataKey::Config) {
            panic!("Already initialized");
        }

        admin.require_auth();

        if total_supply <= 0 {
            panic!("Total supply must be positive");
        }

        if decimals > 18 {
            panic!("Decimals cannot exceed 18");
        }

        let config = TokenConfig {
            name: name.clone(),
            symbol: symbol.clone(),
            decimals,
            total_supply,
            admin: admin.clone(),
            mintable,
            burnable,
            paused: false,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::TotalSupply, &total_supply);
        env.storage().persistent().set(&DataKey::Balance(admin.clone()), &total_supply);

        log!(&env, "Token initialized: {} ({})", name, symbol);
    }

    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        admin.require_auth();
        
        let config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        if !config.mintable {
            panic!("Token not mintable");
        }

        if config.paused {
            panic!("Token is paused");
        }

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let current_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        
        let new_balance = current_balance.checked_add(amount)
            .unwrap_or_else(|| panic!("Balance overflow"));

        let current_total: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
            
        let new_total = current_total.checked_add(amount)
            .unwrap_or_else(|| panic!("Total supply overflow"));

        env.storage().persistent().set(&DataKey::Balance(to.clone()), &new_balance);
        env.storage().instance().set(&DataKey::TotalSupply, &new_total);

        log!(&env, "Minted {} tokens to {}", amount, to);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        
        let config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if !config.burnable {
            panic!("Token not burnable");
        }

        if config.paused {
            panic!("Token is paused");
        }

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let current_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if current_balance < amount {
            panic!("Insufficient balance");
        }

        let new_balance = current_balance - amount;
        let current_total: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        let new_total = current_total - amount;

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &new_balance);
        env.storage().instance().set(&DataKey::TotalSupply, &new_total);

        log!(&env, "Burned {} tokens from {}", amount, from);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.paused {
            panic!("Token is paused");
        }

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        if from == to {
            return; // No-op transfer
        }

        let from_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance.checked_add(amount)
            .unwrap_or_else(|| panic!("Balance overflow"));

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &new_from_balance);
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &new_to_balance);

        log!(&env, "Transferred {} tokens from {} to {}", amount, from, to);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.paused {
            panic!("Token is paused");
        }

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let allowance: i128 = env.storage().temporary()
            .get(&DataKey::Allowance(from.clone(), spender.clone()))
            .unwrap_or(0);

        if allowance < amount {
            panic!("Insufficient allowance");
        }

        let from_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance.checked_add(amount)
            .unwrap_or_else(|| panic!("Balance overflow"));
        let new_allowance = allowance - amount;

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &new_from_balance);
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &new_to_balance);
        env.storage().temporary().set(&DataKey::Allowance(from.clone(), spender.clone()), &new_allowance);

        log!(&env, "Transferred {} tokens from {} to {} via {}", amount, from, to, spender);
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        let config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.paused {
            panic!("Token is paused");
        }

        if amount < 0 {
            panic!("Amount cannot be negative");
        }

        if expiration_ledger <= env.ledger().sequence() {
            panic!("Expiration must be in the future");
        }

        env.storage().temporary().set(&DataKey::Allowance(from.clone(), spender.clone()), &amount);

        log!(&env, "Approved {} tokens from {} to {}", amount, from, spender);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage().temporary()
            .get(&DataKey::Allowance(from, spender))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn decimals(env: Env) -> u32 {
        let config: TokenConfig = env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));
        config.decimals
    }

    pub fn name(env: Env) -> String {
        let config: TokenConfig = env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));
        config.name
    }

    pub fn symbol(env: Env) -> String {
        let config: TokenConfig = env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));
        config.symbol
    }

    pub fn pause(env: Env, admin: Address) {
        admin.require_auth();

        let mut config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        if config.paused {
            panic!("Already paused");
        }

        config.paused = true;
        env.storage().instance().set(&DataKey::Config, &config);

        log!(&env, "Token paused");
    }

    pub fn unpause(env: Env, admin: Address) {
        admin.require_auth();

        let mut config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        if !config.paused {
            panic!("Not paused");
        }

        config.paused = false;
        env.storage().instance().set(&DataKey::Config, &config);

        log!(&env, "Token unpaused");
    }

    pub fn set_admin(env: Env, admin: Address, new_admin: Address) {
        admin.require_auth();

        let mut config: TokenConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        config.admin = new_admin.clone();
        env.storage().instance().set(&DataKey::Config, &config);

        log!(&env, "Admin changed to {}", new_admin);
    }

    pub fn get_config(env: Env) -> TokenConfig {
        env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Ledger}, Env};

    // Import the generated client
    use crate::{TokenContract, TokenContractClient};

    fn create_token_contract(env: &Env) -> Address {
        env.register_contract(None, TokenContract)
    }

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(
            &admin,
            &name,
            &symbol,
            &7u32,
            &1_000_000i128,
            &true,
            &true,
        );

        assert_eq!(client.name(), name);
        assert_eq!(client.symbol(), symbol);
        assert_eq!(client.decimals(), 7u32);
        assert_eq!(client.total_supply(), 1_000_000i128);
        assert_eq!(client.balance(&admin), 1_000_000i128);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_initialize_twice_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);
        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);
    }

    #[test]
    fn test_mint() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        client.mint(&admin, &user, &500_000i128);
        
        assert_eq!(client.balance(&user), 500_000i128);
        assert_eq!(client.total_supply(), 1_500_000i128);
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_mint_unauthorized_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let unauthorized = Address::generate(&env);
        let user = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        client.mint(&unauthorized, &user, &500_000i128);
    }

    #[test]
    #[should_panic(expected = "Token not mintable")]
    fn test_mint_non_mintable_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &false, &true);

        client.mint(&admin, &user, &500_000i128);
    }

    #[test]
    fn test_burn() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        client.burn(&admin, &300_000i128);
        
        assert_eq!(client.balance(&admin), 700_000i128);
        assert_eq!(client.total_supply(), 700_000i128);
    }

    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_burn_insufficient_balance_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        client.burn(&admin, &2_000_000i128);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        client.transfer(&admin, &user, &400_000i128);
        
        assert_eq!(client.balance(&admin), 600_000i128);
        assert_eq!(client.balance(&user), 400_000i128);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        // Approve allowance
        let expiration = env.ledger().sequence() + 100;
        client.approve(&admin, &spender, &200_000i128, &expiration);
        
        assert_eq!(client.allowance(&admin, &spender), 200_000i128);

        // Transfer from allowance
        client.transfer_from(&spender, &admin, &recipient, &150_000i128);
        
        assert_eq!(client.balance(&admin), 850_000i128);
        assert_eq!(client.balance(&recipient), 150_000i128);
        assert_eq!(client.allowance(&admin, &spender), 50_000i128);
    }

    #[test]
    fn test_pause_and_unpause() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_token_contract(&env);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &7u32, &1_000_000i128, &true, &true);

        // Pause token
        client.pause(&admin);
        let config = client.get_config();
        assert_eq!(config.paused, true);

        // Unpause token
        client.unpause(&admin);
        let config = client.get_config();
        assert_eq!(config.paused, false);

        // Transfer should work now
        client.transfer(&admin, &user, &100_000i128);
    }
}