#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Vec};

const DAY_IN_LEDGERS: u32 = 17280; // Approximately 24 hours worth of ledgers

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum VestingType {
    Linear,    // gradual release over time
    Cliff,     // all at once after cliff period
    Hybrid,    // cliff amount + linear remainder
}

#[derive(Clone)]
#[contracttype]
pub struct VestingSchedule {
    pub beneficiary: Address,
    pub token: Address,
    pub total_amount: i128,
    pub start_ledger: u32,
    pub cliff_ledger: u32,
    pub end_ledger: u32,
    pub vesting_type: VestingType,
    pub cliff_amount: i128,
    pub released: i128,
    pub revocable: bool,
    pub revoked: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    Schedule(u64),
    NextScheduleId,
    SchedulesByBeneficiary(Address),
    SchedulesByToken(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct VestingConfig {
    pub admin: Address,
    pub initialized: bool,
}

#[contract]
pub struct VestingContract;

#[contractimpl]
impl VestingContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Config) {
            panic!("Already initialized");
        }

        admin.require_auth();

        let config = VestingConfig {
            admin: admin.clone(),
            initialized: true,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::NextScheduleId, &1u64);

        log!(&env, "Vesting contract initialized by admin: {}", admin);
    }

    pub fn create_schedule(
        env: Env,
        admin: Address,
        beneficiary: Address,
        token: Address,
        total_amount: i128,
        start_ledger: u32,
        cliff_ledger: u32,
        end_ledger: u32,
        vesting_type: VestingType,
        cliff_amount: i128,
        revocable: bool,
    ) -> u64 {
        admin.require_auth();

        let config: VestingConfig = env.storage().instance().get(&DataKey::Config)
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

        if cliff_ledger < start_ledger || cliff_ledger > end_ledger {
            panic!("Invalid cliff ledger");
        }

        // Validate cliff amount based on vesting type
        match vesting_type {
            VestingType::Linear => {
                if cliff_amount != 0 {
                    panic!("Linear vesting cannot have cliff amount");
                }
            }
            VestingType::Cliff => {
                if cliff_amount != total_amount {
                    panic!("Cliff vesting must have cliff_amount = total_amount");
                }
            }
            VestingType::Hybrid => {
                if cliff_amount <= 0 || cliff_amount >= total_amount {
                    panic!("Invalid cliff amount for hybrid vesting");
                }
            }
        }

        let schedule_id: u64 = env.storage().instance()
            .get(&DataKey::NextScheduleId)
            .unwrap_or(1u64);

        let schedule = VestingSchedule {
            beneficiary: beneficiary.clone(),
            token: token.clone(),
            total_amount,
            start_ledger,
            cliff_ledger,
            end_ledger,
            vesting_type,
            cliff_amount,
            released: 0,
            revocable,
            revoked: false,
        };

        // Store the schedule
        env.storage().persistent().set(&DataKey::Schedule(schedule_id), &schedule);

        // Update indices
        let mut beneficiary_schedules: Vec<u64> = env.storage().persistent()
            .get(&DataKey::SchedulesByBeneficiary(beneficiary.clone()))
            .unwrap_or(Vec::new(&env));
        beneficiary_schedules.push_back(schedule_id);
        env.storage().persistent().set(&DataKey::SchedulesByBeneficiary(beneficiary.clone()), &beneficiary_schedules);

        let mut token_schedules: Vec<u64> = env.storage().persistent()
            .get(&DataKey::SchedulesByToken(token.clone()))
            .unwrap_or(Vec::new(&env));
        token_schedules.push_back(schedule_id);
        env.storage().persistent().set(&DataKey::SchedulesByToken(token.clone()), &token_schedules);

        // Increment next ID
        env.storage().instance().set(&DataKey::NextScheduleId, &(schedule_id + 1));

        log!(&env, "Created vesting schedule {} for {} tokens to {}", 
             schedule_id, total_amount, beneficiary);

        schedule_id
    }
    pub fn release(env: Env, schedule_id: u64) {
        let mut schedule: VestingSchedule = env.storage().persistent()
            .get(&DataKey::Schedule(schedule_id))
            .unwrap_or_else(|| panic!("Schedule not found"));

        if schedule.revoked {
            panic!("Schedule revoked");
        }

        let current_ledger = env.ledger().sequence();
        let releasable = Self::calculate_releasable_amount(&env, &schedule, current_ledger);

        if releasable <= 0 {
            panic!("No tokens to release");
        }

        schedule.released += releasable;
        env.storage().persistent().set(&DataKey::Schedule(schedule_id), &schedule);

        // Transfer tokens from contract to beneficiary
        // Note: In practice, this would require the contract to hold tokens or have allowance
        log!(&env, "Released {} tokens from schedule {} to beneficiary {}", 
             releasable, schedule_id, schedule.beneficiary);
    }

    pub fn revoke(env: Env, admin: Address, schedule_id: u64) {
        admin.require_auth();

        let config: VestingConfig = env.storage().instance().get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Not initialized"));

        if config.admin != admin {
            panic!("Unauthorized");
        }

        let mut schedule: VestingSchedule = env.storage().persistent()
            .get(&DataKey::Schedule(schedule_id))
            .unwrap_or_else(|| panic!("Schedule not found"));

        if !schedule.revocable {
            panic!("Schedule not revocable");
        }

        if schedule.revoked {
            panic!("Already revoked");
        }

        schedule.revoked = true;
        env.storage().persistent().set(&DataKey::Schedule(schedule_id), &schedule);

        log!(&env, "Revoked vesting schedule {}", schedule_id);
    }

    pub fn claimable_amount(env: Env, schedule_id: u64) -> i128 {
        let schedule: VestingSchedule = env.storage().persistent()
            .get(&DataKey::Schedule(schedule_id))
            .unwrap_or_else(|| panic!("Schedule not found"));

        if schedule.revoked {
            return 0;
        }

        let current_ledger = env.ledger().sequence();
        Self::calculate_releasable_amount(&env, &schedule, current_ledger)
    }

    pub fn get_schedule(env: Env, schedule_id: u64) -> VestingSchedule {
        env.storage().persistent()
            .get(&DataKey::Schedule(schedule_id))
            .unwrap_or_else(|| panic!("Schedule not found"))
    }

    pub fn get_schedules_by_beneficiary(env: Env, beneficiary: Address) -> Vec<VestingSchedule> {
        let schedule_ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::SchedulesByBeneficiary(beneficiary))
            .unwrap_or(Vec::new(&env));

        let mut schedules = Vec::new(&env);
        for id in schedule_ids.iter() {
            if let Some(schedule) = env.storage().persistent().get(&DataKey::Schedule(id)) {
                schedules.push_back(schedule);
            }
        }
        schedules
    }

    pub fn get_schedules_by_token(env: Env, token: Address) -> Vec<VestingSchedule> {
        let schedule_ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::SchedulesByToken(token))
            .unwrap_or(Vec::new(&env));

        let mut schedules = Vec::new(&env);
        for id in schedule_ids.iter() {
            if let Some(schedule) = env.storage().persistent().get(&DataKey::Schedule(id)) {
                schedules.push_back(schedule);
            }
        }
        schedules
    }

    // Private helper function to calculate releasable amount
    fn calculate_releasable_amount(_env: &Env, schedule: &VestingSchedule, current_ledger: u32) -> i128 {
        if current_ledger < schedule.start_ledger {
            return 0;
        }

        let vested_amount = match schedule.vesting_type {
            VestingType::Linear => {
                if current_ledger >= schedule.end_ledger {
                    schedule.total_amount
                } else {
                    let vesting_duration = schedule.end_ledger - schedule.start_ledger;
                    let elapsed = current_ledger - schedule.start_ledger;
                    (schedule.total_amount * elapsed as i128) / vesting_duration as i128
                }
            }
            VestingType::Cliff => {
                if current_ledger >= schedule.cliff_ledger {
                    schedule.total_amount
                } else {
                    0
                }
            }
            VestingType::Hybrid => {
                let mut vested = 0i128;
                
                // Add cliff amount if cliff period has passed
                if current_ledger >= schedule.cliff_ledger {
                    vested += schedule.cliff_amount;
                }
                
                // Add linear portion
                if current_ledger > schedule.cliff_ledger {
                    let linear_amount = schedule.total_amount - schedule.cliff_amount;
                    let linear_duration = schedule.end_ledger - schedule.cliff_ledger;
                    
                    if current_ledger >= schedule.end_ledger {
                        vested += linear_amount;
                    } else {
                        let elapsed = current_ledger - schedule.cliff_ledger;
                        vested += (linear_amount * elapsed as i128) / linear_duration as i128;
                    }
                }
                
                vested
            }
        };

        // Return amount that can be released (vested - already released)
        if vested_amount > schedule.released {
            vested_amount - schedule.released
        } else {
            0
        }
    }

    pub fn get_config(env: Env) -> VestingConfig {
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
    use crate::{VestingContract, VestingContractClient};

    fn create_vesting_contract(env: &Env) -> Address {
        env.register_contract(None, VestingContract)
    }

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);

        client.initialize(&admin);

        let config = client.get_config();
        assert_eq!(config.admin, admin);
        assert_eq!(config.initialized, true);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_initialize_twice_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.initialize(&admin);
    }

    #[test]
    fn test_create_linear_schedule() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,  // start
            &100u32,  // cliff (same as start for linear)
            &200u32,  // end
            &VestingType::Linear,
            &0i128,   // no cliff amount for linear
            &true     // revocable
        );

        assert_eq!(schedule_id, 1u64);

        let schedule = client.get_schedule(&schedule_id);
        assert_eq!(schedule.beneficiary, beneficiary);
        assert_eq!(schedule.token, token);
        assert_eq!(schedule.total_amount, 1000000i128);
        assert_eq!(schedule.vesting_type, VestingType::Linear);
        assert_eq!(schedule.released, 0i128);
        assert_eq!(schedule.revoked, false);
    }

    #[test]
    fn test_create_cliff_schedule() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,     // start
            &200u32,     // cliff
            &300u32,     // end
            &VestingType::Cliff,
            &1000000i128, // cliff amount = total for cliff vesting
            &false        // non-revocable
        );

        let schedule = client.get_schedule(&schedule_id);
        assert_eq!(schedule.vesting_type, VestingType::Cliff);
        assert_eq!(schedule.cliff_amount, 1000000i128);
        assert_eq!(schedule.revocable, false);
    }

    #[test]
    fn test_create_hybrid_schedule() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,   // start
            &150u32,   // cliff
            &300u32,   // end
            &VestingType::Hybrid,
            &300000i128, // cliff amount (30% of total)
            &true      // revocable
        );

        let schedule = client.get_schedule(&schedule_id);
        assert_eq!(schedule.vesting_type, VestingType::Hybrid);
        assert_eq!(schedule.cliff_amount, 300000i128);
    }

    #[test]
    #[should_panic(expected = "Invalid cliff amount for hybrid vesting")]
    fn test_create_hybrid_invalid_cliff_amount() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        // Try to create hybrid with cliff_amount >= total_amount
        client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,
            &150u32,
            &300u32,
            &VestingType::Hybrid,
            &1000000i128, // cliff amount = total (invalid for hybrid)
            &true
        );
    }

    #[test]
    fn test_claimable_amount_linear() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,  // start
            &100u32,  // cliff
            &200u32,  // end (100 ledger duration)
            &VestingType::Linear,
            &0i128,
            &true
        );

        // Before start: nothing claimable
        env.ledger().with_mut(|li| li.sequence_number = 50);
        assert_eq!(client.claimable_amount(&schedule_id), 0i128);

        // At start: nothing claimable yet
        env.ledger().with_mut(|li| li.sequence_number = 100);
        assert_eq!(client.claimable_amount(&schedule_id), 0i128);

        // Halfway through: 50% claimable
        env.ledger().with_mut(|li| li.sequence_number = 150);
        assert_eq!(client.claimable_amount(&schedule_id), 500000i128);

        // At end: 100% claimable
        env.ledger().with_mut(|li| li.sequence_number = 200);
        assert_eq!(client.claimable_amount(&schedule_id), 1000000i128);

        // After end: still 100%
        env.ledger().with_mut(|li| li.sequence_number = 250);
        assert_eq!(client.claimable_amount(&schedule_id), 1000000i128);
    }

    #[test]
    fn test_claimable_amount_cliff() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,     // start
            &200u32,     // cliff
            &300u32,     // end
            &VestingType::Cliff,
            &1000000i128,
            &true
        );

        // Before cliff: nothing claimable
        env.ledger().with_mut(|li| li.sequence_number = 150);
        assert_eq!(client.claimable_amount(&schedule_id), 0i128);

        // At cliff: everything claimable
        env.ledger().with_mut(|li| li.sequence_number = 200);
        assert_eq!(client.claimable_amount(&schedule_id), 1000000i128);

        // After cliff: still everything
        env.ledger().with_mut(|li| li.sequence_number = 250);
        assert_eq!(client.claimable_amount(&schedule_id), 1000000i128);
    }

    #[test]
    fn test_claimable_amount_hybrid() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,   // start
            &150u32,   // cliff
            &250u32,   // end
            &VestingType::Hybrid,
            &300000i128, // 30% cliff
            &true
        );

        // Before cliff: nothing
        env.ledger().with_mut(|li| li.sequence_number = 120);
        assert_eq!(client.claimable_amount(&schedule_id), 0i128);

        // At cliff: cliff amount
        env.ledger().with_mut(|li| li.sequence_number = 150);
        assert_eq!(client.claimable_amount(&schedule_id), 300000i128);

        // Halfway through linear portion: cliff + 50% of remaining
        env.ledger().with_mut(|li| li.sequence_number = 200);
        // Remaining after cliff: 700000
        // Half of remaining: 350000
        // Total: 300000 + 350000 = 650000
        assert_eq!(client.claimable_amount(&schedule_id), 650000i128);

        // At end: everything
        env.ledger().with_mut(|li| li.sequence_number = 250);
        assert_eq!(client.claimable_amount(&schedule_id), 1000000i128);
    }

    #[test]
    fn test_revoke_schedule() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,
            &100u32,
            &200u32,
            &VestingType::Linear,
            &0i128,
            &true  // revocable
        );

        // Revoke the schedule
        client.revoke(&admin, &schedule_id);

        let schedule = client.get_schedule(&schedule_id);
        assert_eq!(schedule.revoked, true);

        // After revocation, nothing is claimable
        env.ledger().with_mut(|li| li.sequence_number = 150);
        assert_eq!(client.claimable_amount(&schedule_id), 0i128);
    }

    #[test]
    #[should_panic(expected = "Schedule not revocable")]
    fn test_revoke_non_revocable_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token = Address::generate(&env);

        client.initialize(&admin);

        let schedule_id = client.create_schedule(
            &admin,
            &beneficiary,
            &token,
            &1000000i128,
            &100u32,
            &100u32,
            &200u32,
            &VestingType::Linear,
            &0i128,
            &false  // not revocable
        );

        client.revoke(&admin, &schedule_id);
    }

    #[test]
    fn test_get_schedules_by_beneficiary() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = create_vesting_contract(&env);
        let client = VestingContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let token1 = Address::generate(&env);
        let token2 = Address::generate(&env);

        client.initialize(&admin);

        // Create two schedules for the same beneficiary
        client.create_schedule(&admin, &beneficiary, &token1, &1000000i128, &100u32, &100u32, &200u32, &VestingType::Linear, &0i128, &true);
        client.create_schedule(&admin, &beneficiary, &token2, &2000000i128, &100u32, &150u32, &300u32, &VestingType::Cliff, &2000000i128, &false);

        let schedules = client.get_schedules_by_beneficiary(&beneficiary);
        assert_eq!(schedules.len(), 2);
        assert_eq!(schedules.get(0).unwrap().total_amount, 1000000i128);
        assert_eq!(schedules.get(1).unwrap().total_amount, 2000000i128);
    }
}