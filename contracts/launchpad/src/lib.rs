#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Vec, String};

#[derive(Clone)]
#[contracttype]
pub struct TokenLaunch {
    pub id: u64,
    pub creator: Address,
    pub token_contract: Address,
    pub name: String,
    pub symbol: String,
    pub total_supply: i128,
    pub launch_ledger: u32,
    pub vesting_contract: Option<Address>,
    pub airdrop_contract: Option<Address>,
    pub website: String,
    pub description: String,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    Launch(u64),
    NextLaunchId,
    LaunchesByCreator(Address),
    AllLaunches,
    TotalLaunches,
}

#[derive(Clone)]
#[contracttype]
pub struct LaunchpadConfig {
    pub admin: Address,
    pub initialized: bool,
}

#[contract]
pub struct LaunchpadContract;

#[contractimpl]
impl LaunchpadContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Config) {
            panic!("Already initialized");
        }

        admin.require_auth();

        let config = LaunchpadConfig {
            admin: admin.clone(),
            initialized: true,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::NextLaunchId, &1u64);
        env.storage().instance().set(&DataKey::TotalLaunches, &0u64);

        // Initialize empty all launches vector
        let all_launches = Vec::<u64>::new(&env);
        env.storage().persistent().set(&DataKey::AllLaunches, &all_launches);

        log!(&env, "Launchpad registry initialized by admin: {}", admin);
    }

    pub fn register_launch(
        env: Env,
        creator: Address,
        token_contract: Address,
        name: String,
        symbol: String,
        total_supply: i128,
        vesting_contract: Option<Address>,
        airdrop_contract: Option<Address>,
        website: String,
        description: String,
    ) -> u64 {
        creator.require_auth();

        let config: LaunchpadConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if !config.initialized {
            panic!("Not initialized");
        }

        if total_supply <= 0 {
            panic!("Total supply must be positive");
        }

        if name.len() == 0 || symbol.len() == 0 {
            panic!("Name and symbol cannot be empty");
        }

        let launch_id: u64 = env.storage().instance()
            .get(&DataKey::NextLaunchId)
            .unwrap_or(1u64);

        let current_ledger = env.ledger().sequence();

        let launch = TokenLaunch {
            id: launch_id,
            creator: creator.clone(),
            token_contract: token_contract.clone(),
            name: name.clone(),
            symbol: symbol.clone(),
            total_supply,
            launch_ledger: current_ledger,
            vesting_contract,
            airdrop_contract,
            website,
            description,
            active: true,
        };

        env.storage().persistent().set(&DataKey::Launch(launch_id), &launch);

        // Update creator's launches list
        let mut creator_launches: Vec<u64> = env.storage().persistent()
            .get(&DataKey::LaunchesByCreator(creator.clone()))
            .unwrap_or(Vec::new(&env));
        creator_launches.push_back(launch_id);
        env.storage().persistent().set(&DataKey::LaunchesByCreator(creator.clone()), &creator_launches);

        // Update all launches list
        let mut all_launches: Vec<u64> = env.storage().persistent()
            .get(&DataKey::AllLaunches)
            .unwrap_or(Vec::new(&env));
        all_launches.push_back(launch_id);
        env.storage().persistent().set(&DataKey::AllLaunches, &all_launches);

        // Increment counters
        env.storage().instance().set(&DataKey::NextLaunchId, &(launch_id + 1));
        
        let total_launches: u64 = env.storage().instance()
            .get(&DataKey::TotalLaunches)
            .unwrap_or(0u64) + 1;
        env.storage().instance().set(&DataKey::TotalLaunches, &total_launches);

        log!(&env, "Registered token launch {} for creator {} with token contract {}", 
             launch_id, creator, token_contract);

        launch_id
    }

    pub fn get_launch(env: Env, launch_id: u64) -> TokenLaunch {
        env.storage().persistent()
            .get(&DataKey::Launch(launch_id))
            .unwrap_or_else(|| panic!("Launch not found"))
    }

    pub fn get_launches_by_creator(env: Env, creator: Address) -> Vec<TokenLaunch> {
        let launch_ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::LaunchesByCreator(creator))
            .unwrap_or(Vec::new(&env));

        let mut launches = Vec::new(&env);
        for i in 0..launch_ids.len() {
            let launch_id = launch_ids.get(i).unwrap();
            if let Some(launch) = env.storage().persistent().get(&DataKey::Launch(launch_id)) {
                launches.push_back(launch);
            }
        }
        launches
    }

    pub fn get_all_launches(env: Env) -> Vec<TokenLaunch> {
        let launch_ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::AllLaunches)
            .unwrap_or(Vec::new(&env));

        let mut launches = Vec::new(&env);
        for i in 0..launch_ids.len() {
            let launch_id = launch_ids.get(i).unwrap();
            if let Some(launch) = env.storage().persistent().get(&DataKey::Launch(launch_id)) {
                launches.push_back(launch);
            }
        }
        launches
    }

    pub fn deactivate_launch(env: Env, admin: Address, launch_id: u64) {
        admin.require_auth();

        let config: LaunchpadConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        let mut launch: TokenLaunch = env.storage().persistent()
            .get(&DataKey::Launch(launch_id))
            .unwrap_or_else(|| panic!("Launch not found"));

        if !launch.active {
            panic!("Launch is already inactive");
        }

        launch.active = false;
        env.storage().persistent().set(&DataKey::Launch(launch_id), &launch);

        log!(&env, "Launch {} deactivated by admin {}", launch_id, admin);
    }

    pub fn get_total_launches(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::TotalLaunches)
            .unwrap_or(0u64)
    }

    pub fn update_launch_contracts(
        env: Env,
        creator: Address,
        launch_id: u64,
        vesting_contract: Option<Address>,
        airdrop_contract: Option<Address>,
    ) {
        creator.require_auth();

        let mut launch: TokenLaunch = env.storage().persistent()
            .get(&DataKey::Launch(launch_id))
            .unwrap_or_else(|| panic!("Launch not found"));

        if launch.creator != creator {
            panic!("Unauthorized");
        }

        launch.vesting_contract = vesting_contract;
        launch.airdrop_contract = airdrop_contract;

        env.storage().persistent().set(&DataKey::Launch(launch_id), &launch);

        log!(&env, "Updated contracts for launch {} by creator {}", launch_id, creator);
    }

    pub fn get_active_launches(env: Env) -> Vec<TokenLaunch> {
        let launch_ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::AllLaunches)
            .unwrap_or(Vec::new(&env));

        let mut active_launches = Vec::new(&env);
        for i in 0..launch_ids.len() {
            let launch_id = launch_ids.get(i).unwrap();
            if let Some(launch) = env.storage().persistent().get::<DataKey, TokenLaunch>(&DataKey::Launch(launch_id)) {
                if launch.active {
                    active_launches.push_back(launch);
                }
            }
        }
        active_launches
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::{Address as _}, Env, Address, String};

    #[allow(dead_code)]
    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);

        let config: LaunchpadConfig = env.as_contract(&contract_id, || {
            env.storage().instance().get(&DataKey::Config).unwrap()
        });
        assert_eq!(config.admin, admin);
        assert!(config.initialized);
        assert_eq!(client.get_total_launches(), 0);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_initialize_twice_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.initialize(&admin); // Should panic
    }

    #[test]
    fn test_register_launch() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let token_contract = Address::generate(&env);

        client.initialize(&admin);

        let launch_id = client.register_launch(
            &creator,
            &token_contract,
            &String::from_str(&env, "MyToken"),
            &String::from_str(&env, "MTK"),
            &1000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://mytoken.com"),
            &String::from_str(&env, "My awesome token"),
        );

        assert_eq!(launch_id, 1);
        assert_eq!(client.get_total_launches(), 1);

        let launch = client.get_launch(&launch_id);
        assert_eq!(launch.creator, creator);
        assert_eq!(launch.token_contract, token_contract);
        assert_eq!(launch.name, String::from_str(&env, "MyToken"));
        assert_eq!(launch.symbol, String::from_str(&env, "MTK"));
        assert_eq!(launch.total_supply, 1000000);
        assert!(launch.active);
    }

    #[test]
    #[should_panic(expected = "Total supply must be positive")]
    fn test_register_launch_invalid_supply() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let token_contract = Address::generate(&env);

        client.initialize(&admin);

        client.register_launch(
            &creator,
            &token_contract,
            &String::from_str(&env, "MyToken"),
            &String::from_str(&env, "MTK"),
            &0i128, // Invalid supply
            &None,
            &None,
            &String::from_str(&env, "https://mytoken.com"),
            &String::from_str(&env, "My token"),
        );
    }

    #[test]
    fn test_get_launches_by_creator() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let other_creator = Address::generate(&env);

        client.initialize(&admin);

        // Register launches for creator
        let _launch_id_1 = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "Token1"),
            &String::from_str(&env, "TK1"),
            &1000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://token1.com"),
            &String::from_str(&env, "Token 1"),
        );

        let launch_id_2 = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "Token2"),
            &String::from_str(&env, "TK2"),
            &2000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://token2.com"),
            &String::from_str(&env, "Token 2"),
        );

        // Register launch for other creator
        client.register_launch(
            &other_creator,
            &Address::generate(&env),
            &String::from_str(&env, "OtherToken"),
            &String::from_str(&env, "OTK"),
            &500000i128,
            &None,
            &None,
            &String::from_str(&env, "https://other.com"),
            &String::from_str(&env, "Other token"),
        );

        let creator_launches = client.get_launches_by_creator(&creator);
        assert_eq!(creator_launches.len(), 2);
        
        let other_launches = client.get_launches_by_creator(&other_creator);
        assert_eq!(other_launches.len(), 1);
    }

    #[test]
    fn test_get_all_launches() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator1 = Address::generate(&env);
        let creator2 = Address::generate(&env);

        client.initialize(&admin);

        // Register multiple launches
        for i in 0..3 {
            let creator = if i % 2 == 0 { &creator1 } else { &creator2 };
            let token_name = if i == 0 { "Token0" } else if i == 1 { "Token1" } else { "Token2" };
            let token_symbol = if i == 0 { "TK0" } else if i == 1 { "TK1" } else { "TK2" };
            
            client.register_launch(
                creator,
                &Address::generate(&env),
                &String::from_str(&env, token_name),
                &String::from_str(&env, token_symbol),
                &(1000000i128 * (i as i128 + 1)),
                &None,
                &None,
                &String::from_str(&env, "https://example.com"),
                &String::from_str(&env, "Test token"),
            );
        }

        let all_launches = client.get_all_launches();
        assert_eq!(all_launches.len(), 3);
        assert_eq!(client.get_total_launches(), 3);
    }

    #[test]
    fn test_deactivate_launch() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);

        client.initialize(&admin);

        let launch_id = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "MyToken"),
            &String::from_str(&env, "MTK"),
            &1000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://mytoken.com"),
            &String::from_str(&env, "My token"),
        );

        client.deactivate_launch(&admin, &launch_id);

        let launch = client.get_launch(&launch_id);
        assert!(!launch.active);
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_deactivate_launch_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let unauthorized = Address::generate(&env);

        client.initialize(&admin);

        let launch_id = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "MyToken"),
            &String::from_str(&env, "MTK"),
            &1000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://mytoken.com"),
            &String::from_str(&env, "My token"),
        );

        client.deactivate_launch(&unauthorized, &launch_id); // Should panic
    }

    #[test]
    fn test_update_launch_contracts() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let vesting_contract = Address::generate(&env);
        let airdrop_contract = Address::generate(&env);

        client.initialize(&admin);

        let launch_id = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "MyToken"),
            &String::from_str(&env, "MTK"),
            &1000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://mytoken.com"),
            &String::from_str(&env, "My token"),
        );

        client.update_launch_contracts(
            &creator,
            &launch_id,
            &Some(vesting_contract.clone()),
            &Some(airdrop_contract.clone()),
        );

        let launch = client.get_launch(&launch_id);
        assert_eq!(launch.vesting_contract, Some(vesting_contract));
        assert_eq!(launch.airdrop_contract, Some(airdrop_contract));
    }

    #[test]
    fn test_get_active_launches() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, LaunchpadContract);
        let client = LaunchpadContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);

        client.initialize(&admin);

        // Register 3 launches
        let launch_id_1 = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "Token1"),
            &String::from_str(&env, "TK1"),
            &1000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://token1.com"),
            &String::from_str(&env, "Token 1"),
        );

        let launch_id_2 = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "Token2"),
            &String::from_str(&env, "TK2"),
            &2000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://token2.com"),
            &String::from_str(&env, "Token 2"),
        );

        let launch_id_3 = client.register_launch(
            &creator,
            &Address::generate(&env),
            &String::from_str(&env, "Token3"),
            &String::from_str(&env, "TK3"),
            &3000000i128,
            &None,
            &None,
            &String::from_str(&env, "https://token3.com"),
            &String::from_str(&env, "Token 3"),
        );

        // Deactivate one launch
        client.deactivate_launch(&admin, &launch_id_2);

        // Check active launches
        let active_launches = client.get_active_launches();
        assert_eq!(active_launches.len(), 2);

        let all_launches = client.get_all_launches();
        assert_eq!(all_launches.len(), 3);
    }
}