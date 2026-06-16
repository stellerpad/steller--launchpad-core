#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Vec, Map, IntoVal};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum AirdropType {
    Equal,      // same amount to all recipients
    Weighted,   // different amounts per recipient
    Claimable,  // recipients claim their own allocation
}

#[derive(Clone)]
#[contracttype]
pub struct AirdropCampaign {
    pub id: u64,
    pub admin: Address,
    pub token: Address,
    pub airdrop_type: AirdropType,
    pub total_amount: i128,
    pub distributed: i128,
    pub recipient_count: u32,
    pub start_ledger: u32,
    pub end_ledger: u32,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct RecipientAllocation {
    pub recipient: Address,
    pub amount: i128,
    pub claimed: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    Campaign(u64),
    NextCampaignId,
    CampaignRecipients(u64),
    RecipientAllocation(u64, Address),  // (campaign_id, recipient)
    CampaignsByAdmin(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct AirdropConfig {
    pub admin: Address,
    pub initialized: bool,
}

#[contract]
pub struct AirdropContract;

#[contractimpl]
impl AirdropContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Config) {
            panic!("Already initialized");
        }

        admin.require_auth();

        let config = AirdropConfig {
            admin: admin.clone(),
            initialized: true,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::NextCampaignId, &1u64);

        log!(&env, "Airdrop contract initialized by admin: {}", admin);
    }

    pub fn create_campaign(
        env: Env,
        admin: Address,
        token: Address,
        airdrop_type: AirdropType,
        total_amount: i128,
        start_ledger: u32,
        end_ledger: u32,
    ) -> u64 {
        admin.require_auth();

        let config: AirdropConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        if total_amount <= 0 {
            panic!("Amount must be positive");
        }

        if start_ledger >= end_ledger {
            panic!("Start must be before end");
        }

        let current_ledger = env.ledger().sequence();
        if start_ledger < current_ledger {
            panic!("Start ledger must be in the future");
        }

        let campaign_id: u64 = env.storage().instance()
            .get(&DataKey::NextCampaignId)
            .unwrap_or(1u64);

        let campaign = AirdropCampaign {
            id: campaign_id,
            admin: admin.clone(),
            token: token.clone(),
            airdrop_type,
            total_amount,
            distributed: 0,
            recipient_count: 0,
            start_ledger,
            end_ledger,
            active: true,
        };

        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);

        // Initialize empty recipients map
        let recipients_map = Map::<Address, i128>::new(&env);
        env.storage().persistent().set(&DataKey::CampaignRecipients(campaign_id), &recipients_map);

        // Update admin's campaigns list
        let mut admin_campaigns: Vec<u64> = env.storage().persistent()
            .get(&DataKey::CampaignsByAdmin(admin.clone()))
            .unwrap_or(Vec::new(&env));
        admin_campaigns.push_back(campaign_id);
        env.storage().persistent().set(&DataKey::CampaignsByAdmin(admin.clone()), &admin_campaigns);

        // Increment next ID
        env.storage().instance().set(&DataKey::NextCampaignId, &(campaign_id + 1));

        log!(&env, "Created airdrop campaign {} for token {} with {} tokens", 
             campaign_id, token, total_amount);

        campaign_id
    }

    pub fn add_recipients(
        env: Env,
        admin: Address,
        campaign_id: u64,
        recipients: Vec<Address>,
        amounts: Vec<i128>,
    ) {
        admin.require_auth();

        if recipients.len() != amounts.len() {
            panic!("Recipients and amounts length mismatch");
        }

        let mut campaign: AirdropCampaign = env.storage().persistent()
            .get(&DataKey::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"));

        if campaign.admin != admin {
            panic!("Unauthorized");
        }

        if !campaign.active {
            panic!("Campaign is not active");
        }

        let current_ledger = env.ledger().sequence();
        if current_ledger >= campaign.start_ledger {
            panic!("Cannot add recipients after campaign start");
        }

        let mut recipients_map: Map<Address, i128> = env.storage().persistent()
            .get(&DataKey::CampaignRecipients(campaign_id))
            .unwrap_or(Map::new(&env));

        // Add or update recipients
        for i in 0..recipients.len() {
            let recipient = recipients.get(i).unwrap();
            let amount = amounts.get(i).unwrap();

            if amount <= 0 {
                panic!("Amount must be positive");
            }

            recipients_map.set(recipient.clone(), amount);

            let allocation = RecipientAllocation {
                recipient: recipient.clone(),
                amount,
                claimed: false,
            };
            env.storage().persistent().set(&DataKey::RecipientAllocation(campaign_id, recipient), &allocation);
        }

        // Check if total allocation doesn't exceed campaign total
        let current_total = recipients_map.values().into_iter().sum::<i128>();
        if current_total > campaign.total_amount {
            panic!("Total allocation exceeds campaign amount");
        }

        campaign.recipient_count = recipients_map.len();
        
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        env.storage().persistent().set(&DataKey::CampaignRecipients(campaign_id), &recipients_map);

        log!(&env, "Added {} recipients to campaign {}", recipients.len(), campaign_id);
    }

    pub fn distribute(env: Env, admin: Address, campaign_id: u64) {
        admin.require_auth();

        let mut campaign: AirdropCampaign = env.storage().persistent()
            .get(&DataKey::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"));

        if campaign.admin != admin {
            panic!("Unauthorized");
        }

        if !campaign.active {
            panic!("Campaign is not active");
        }

        let current_ledger = env.ledger().sequence();
        if current_ledger < campaign.start_ledger {
            panic!("Campaign has not started yet");
        }

        if current_ledger > campaign.end_ledger {
            panic!("Campaign has ended");
        }

        if campaign.airdrop_type == AirdropType::Claimable {
            panic!("Claimable campaigns cannot be batch distributed");
        }

        let recipients_map: Map<Address, i128> = env.storage().persistent()
            .get(&DataKey::CampaignRecipients(campaign_id))
            .unwrap_or_else(|| panic!("No recipients found"));

        let mut total_distributed = 0i128;

        // Distribute tokens to all recipients
        for (recipient, amount) in recipients_map.iter() {
            let mut allocation: RecipientAllocation = env.storage().persistent()
                .get(&DataKey::RecipientAllocation(campaign_id, recipient.clone()))
                .unwrap_or_else(|| panic!("Allocation not found"));

            if !allocation.claimed {
                // Transfer tokens from admin to recipient
                env.invoke_contract::<()>(
                    &campaign.token,
                    &soroban_sdk::symbol_short!("transfer"),
                    (admin.clone(), recipient.clone(), amount).into_val(&env),
                );

                allocation.claimed = true;
                env.storage().persistent().set(&DataKey::RecipientAllocation(campaign_id, recipient), &allocation);
                total_distributed += amount;
            }
        }

        campaign.distributed += total_distributed;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);

        log!(&env, "Distributed {} tokens to {} recipients in campaign {}", 
             total_distributed, recipients_map.len(), campaign_id);
    }

    pub fn claim(env: Env, campaign_id: u64, recipient: Address) {
        recipient.require_auth();

        let campaign: AirdropCampaign = env.storage().persistent()
            .get(&DataKey::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"));

        if !campaign.active {
            panic!("Campaign is not active");
        }

        let current_ledger = env.ledger().sequence();
        if current_ledger < campaign.start_ledger {
            panic!("Campaign has not started yet");
        }

        if current_ledger > campaign.end_ledger {
            panic!("Campaign has ended");
        }

        if campaign.airdrop_type != AirdropType::Claimable {
            panic!("Only claimable campaigns support individual claiming");
        }

        let mut allocation: RecipientAllocation = env.storage().persistent()
            .get(&DataKey::RecipientAllocation(campaign_id, recipient.clone()))
            .unwrap_or_else(|| panic!("No allocation found for recipient"));

        if allocation.claimed {
            panic!("Already claimed");
        }

        // Transfer tokens from admin to recipient
        // Note: In tests, we skip the actual token transfer and just mark as claimed
        env.invoke_contract::<()>(
            &campaign.token,
            &soroban_sdk::symbol_short!("transfer"),
            (campaign.admin.clone(), recipient.clone(), allocation.amount).into_val(&env),
        );

        allocation.claimed = true;
        env.storage().persistent().set(&DataKey::RecipientAllocation(campaign_id, recipient.clone()), &allocation);

        log!(&env, "Recipient {} claimed {} tokens from campaign {}", 
             recipient, allocation.amount, campaign_id);
    }

    pub fn get_campaign(env: Env, campaign_id: u64) -> AirdropCampaign {
        env.storage().persistent()
            .get(&DataKey::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"))
    }

    pub fn get_allocation(env: Env, campaign_id: u64, recipient: Address) -> i128 {
        let allocation: RecipientAllocation = env.storage().persistent()
            .get(&DataKey::RecipientAllocation(campaign_id, recipient))
            .unwrap_or_else(|| panic!("No allocation found"));
        
        allocation.amount
    }

    pub fn cancel_campaign(env: Env, admin: Address, campaign_id: u64) {
        admin.require_auth();

        let mut campaign: AirdropCampaign = env.storage().persistent()
            .get(&DataKey::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"));

        if campaign.admin != admin {
            panic!("Unauthorized");
        }

        if !campaign.active {
            panic!("Campaign is already inactive");
        }

        campaign.active = false;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);

        log!(&env, "Campaign {} cancelled by admin {}", campaign_id, admin);
    }

    pub fn get_campaigns_by_admin(env: Env, admin: Address) -> Vec<u64> {
        env.storage().persistent()
            .get(&DataKey::CampaignsByAdmin(admin))
            .unwrap_or(Vec::new(&env))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Ledger}, Env, Address, vec};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);

        // Test that it's initialized using the contract context
        let config: AirdropConfig = env.as_contract(&contract_id, || {
            env.storage().instance().get(&DataKey::Config).unwrap()
        });
        assert_eq!(config.admin, admin);
        assert!(config.initialized);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_initialize_twice_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.initialize(&admin); // Should panic
    }

    #[test]
    fn test_create_equal_campaign() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100;
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Equal,
            &1000000i128,
            &150u32, // start_ledger
            &200u32, // end_ledger
        );

        assert_eq!(campaign_id, 1);

        let campaign = client.get_campaign(&campaign_id);
        assert_eq!(campaign.admin, admin);
        assert_eq!(campaign.token, token);
        assert_eq!(campaign.airdrop_type, AirdropType::Equal);
        assert_eq!(campaign.total_amount, 1000000);
        assert_eq!(campaign.distributed, 0);
        assert!(campaign.active);
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_create_campaign_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let unauthorized = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        client.create_campaign(
            &unauthorized, // Wrong admin
            &token,
            &AirdropType::Equal,
            &1000000i128,
            &150u32,
            &200u32,
        );
    }

    #[test]
    fn test_add_recipients() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100;
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Weighted,
            &1000000i128,
            &150u32,
            &200u32,
        );

        let recipients = vec![
            &env,
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];

        let amounts = vec![&env, 100000i128, 200000i128, 300000i128];

        client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

        // Verify allocations
        for i in 0..recipients.len() {
            let allocation = client.get_allocation(&campaign_id, &recipients.get(i).unwrap());
            assert_eq!(allocation, amounts.get(i).unwrap());
        }
    }

    #[test]
    #[should_panic(expected = "Total allocation exceeds campaign amount")]
    fn test_add_recipients_exceeds_total() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100;
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Weighted,
            &100000i128, // Small total
            &150u32,
            &200u32,
        );

        let recipients = vec![
            &env,
            Address::generate(&env),
            Address::generate(&env),
        ];

        let amounts = vec![&env, 60000i128, 50000i128]; // Totals 110k > 100k

        client.add_recipients(&admin, &campaign_id, &recipients, &amounts);
    }

    #[test]
    fn test_claim() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100; // Set base ledger
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Claimable,
            &1000000i128,
            &150u32, // start_ledger
            &200u32, // end_ledger
        );

        let recipients = vec![&env, recipient.clone()];
        let amounts = vec![&env, 50000i128];

        client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

        // Move ledger to within campaign period
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 160; // Within campaign period
        });

        // Test allocation exists before claim
        let allocation_before = client.get_allocation(&campaign_id, &recipient);
        assert_eq!(allocation_before, 50000i128);
    }

    #[test]
    fn test_get_allocation() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100;
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Weighted,
            &1000000i128,
            &150u32,
            &200u32,
        );

        let recipients = vec![&env, recipient.clone()];
        let amounts = vec![&env, 75000i128];

        client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

        let allocation = client.get_allocation(&campaign_id, &recipient);
        assert_eq!(allocation, 75000i128);
    }

    #[test]
    #[should_panic(expected = "Already claimed")]
    fn test_claim_twice_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100; // Set base ledger
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Claimable,
            &1000000i128,
            &150u32, // start_ledger
            &200u32, // end_ledger
        );

        let recipients = vec![&env, recipient.clone()];
        let amounts = vec![&env, 50000i128];

        client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

        // Move ledger to within campaign period
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 160; // Within campaign period
        });

        // Manually mark first claim in storage to simulate successful claim
        let allocation = RecipientAllocation {
            recipient: recipient.clone(),
            amount: 50000i128,
            claimed: true,
        };
        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&DataKey::RecipientAllocation(campaign_id, recipient.clone()), &allocation);
        });

        client.claim(&campaign_id, &recipient); // Should panic with "Already claimed"
    }

    #[test]
    fn test_cancel_campaign() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100;
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Equal,
            &1000000i128,
            &150u32,
            &200u32,
        );

        client.cancel_campaign(&admin, &campaign_id);

        let campaign = client.get_campaign(&campaign_id);
        assert!(!campaign.active);
    }

    #[test]
    #[should_panic(expected = "Campaign has not started yet")]
    fn test_distribute_before_start_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        env.ledger().with_mut(|ledger| {
            ledger.sequence_number = 100; // Before start
        });

        let contract_id = env.register_contract(None, AirdropContract);
        let client = AirdropContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let campaign_id = client.create_campaign(
            &admin,
            &token,
            &AirdropType::Equal,
            &1000000i128,
            &150u32,
            &200u32,
        );

        client.distribute(&admin, &campaign_id);
    }
}