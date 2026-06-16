# API Reference

This document provides a comprehensive reference for all smart contract functions in Stellar Launchpad Core.

## Token Contract

### Public Functions

#### `initialize(admin: Address, decimals: u32, name: String, symbol: String, mintable: bool, pausable: bool)`
Initializes a new token contract.

**Parameters:**
- `admin`: Administrator address
- `decimals`: Number of decimal places (typically 7 for Stellar)
- `name`: Token name (e.g., "My Token")
- `symbol`: Token symbol (e.g., "MTK")
- `mintable`: Whether new tokens can be minted
- `pausable`: Whether the token can be paused

**Authorization:** None (called once during deployment)

#### `mint(admin: Address, to: Address, amount: i128)`
Mints new tokens to specified address.

**Parameters:**
- `admin`: Administrator address (must match contract admin)
- `to`: Recipient address  
- `amount`: Number of tokens to mint

**Authorization:** `admin.require_auth()`

#### `burn(from: Address, amount: i128)`
Burns tokens from specified address.

**Parameters:**
- `from`: Address to burn tokens from
- `amount`: Number of tokens to burn

**Authorization:** `from.require_auth()`

#### `pause(admin: Address)`
Pauses all token transfers (if contract is pausable).

**Authorization:** `admin.require_auth()`

#### `unpause(admin: Address)`
Resumes token transfers.

**Authorization:** `admin.require_auth()`

### View Functions

#### `name() -> String`
Returns the token name.

#### `symbol() -> String`
Returns the token symbol.

#### `decimals() -> u32`
Returns number of decimal places.

#### `total_supply() -> i128`
Returns total token supply.

#### `balance(id: Address) -> i128`
Returns balance for given address.

#### `allowance(from: Address, spender: Address) -> i128`
Returns spending allowance.

## Vesting Contract

### Public Functions

#### `initialize(admin: Address)`
Initializes the vesting contract.

**Parameters:**
- `admin`: Administrator address

#### `create_schedule(admin: Address, beneficiary: Address, token: Address, amount: i128, vesting_type: VestingType, start_time: u64, cliff_time: u64, end_time: u64, cliff_amount: Option<i128>, revocable: bool) -> u64`
Creates a new vesting schedule.

**Parameters:**
- `admin`: Administrator address
- `beneficiary`: Address that will receive vested tokens
- `token`: Token contract address
- `amount`: Total amount to vest
- `vesting_type`: Linear, Cliff, or Hybrid
- `start_time`: Vesting start timestamp
- `cliff_time`: Cliff period end timestamp  
- `end_time`: Vesting completion timestamp
- `cliff_amount`: Amount released at cliff (for Hybrid type)
- `revocable`: Whether schedule can be revoked

**Returns:** Schedule ID
**Authorization:** `admin.require_auth()`

#### `release_tokens(beneficiary: Address, schedule_id: u64)`
Releases vested tokens for a schedule.

**Parameters:**
- `beneficiary`: Beneficiary address
- `schedule_id`: Vesting schedule ID

**Authorization:** `beneficiary.require_auth()`

#### `revoke_schedule(admin: Address, schedule_id: u64)`
Revokes a vesting schedule (if revocable).

**Parameters:**
- `admin`: Administrator address
- `schedule_id`: Schedule ID to revoke

**Authorization:** `admin.require_auth()`

### View Functions

#### `get_claimable_amount(schedule_id: u64) -> i128`
Returns amount available to claim for a schedule.

#### `get_schedule(schedule_id: u64) -> VestingSchedule`
Returns full schedule details.

#### `get_schedules_by_beneficiary(beneficiary: Address) -> Vec<u64>`
Returns all schedule IDs for a beneficiary.

## Airdrop Contract

### Public Functions

#### `initialize(admin: Address)`
Initializes the airdrop contract.

#### `create_campaign(admin: Address, token: Address, airdrop_type: AirdropType, total_amount: i128, recipients: Vec<Address>, amounts: Option<Vec<i128>>, start_time: u64, end_time: u64) -> u64`
Creates a new airdrop campaign.

**Parameters:**
- `admin`: Administrator address
- `token`: Token contract address
- `airdrop_type`: Equal, Weighted, or Claimable
- `total_amount`: Total tokens for distribution
- `recipients`: List of recipient addresses
- `amounts`: Individual amounts (for Weighted type)
- `start_time`: Campaign start timestamp
- `end_time`: Campaign end timestamp

**Returns:** Campaign ID
**Authorization:** `admin.require_auth()`

#### `add_recipients(admin: Address, campaign_id: u64, recipients: Vec<Address>, amounts: Option<Vec<i128>>)`
Adds recipients to existing campaign.

**Authorization:** `admin.require_auth()`

#### `distribute(admin: Address, campaign_id: u64)`
Distributes tokens for Equal/Weighted campaigns.

**Authorization:** `admin.require_auth()`

#### `claim(recipient: Address, campaign_id: u64)`
Claims tokens for Claimable campaigns.

**Authorization:** `recipient.require_auth()`

#### `cancel_campaign(admin: Address, campaign_id: u64)`
Cancels an active campaign.

**Authorization:** `admin.require_auth()`

### View Functions

#### `get_campaign(campaign_id: u64) -> AirdropCampaign`
Returns campaign details.

#### `get_allocation(campaign_id: u64, recipient: Address) -> i128`
Returns allocation amount for recipient.

## Launchpad Registry Contract

### Public Functions

#### `initialize(admin: Address)`
Initializes the registry contract.

#### `register_launch(creator: Address, token: Address, name: String, symbol: String, total_supply: i128, vesting_contract: Option<Address>, airdrop_contract: Option<Address>, website: String, description: String) -> u64`
Registers a new token launch.

**Parameters:**
- `creator`: Launch creator address
- `token`: Token contract address
- `name`: Token name
- `symbol`: Token symbol
- `total_supply`: Total token supply
- `vesting_contract`: Optional vesting contract address
- `airdrop_contract`: Optional airdrop contract address
- `website`: Project website URL
- `description`: Launch description

**Returns:** Launch ID
**Authorization:** `creator.require_auth()`

#### `update_launch_contracts(admin: Address, launch_id: u64, vesting_contract: Option<Address>, airdrop_contract: Option<Address>)`
Updates contract addresses for a launch.

**Authorization:** `admin.require_auth()`

#### `deactivate_launch(admin: Address, launch_id: u64)`
Deactivates a launch.

**Authorization:** `admin.require_auth()`

### View Functions

#### `get_launch(launch_id: u64) -> Launch`
Returns launch details.

#### `get_launches_by_creator(creator: Address) -> Vec<u64>`
Returns launch IDs for a creator.

#### `get_all_launches() -> Vec<u64>`
Returns all launch IDs.

#### `get_active_launches() -> Vec<u64>`
Returns only active launch IDs.

## Data Types

### VestingType
```rust
enum VestingType {
    Linear,    // Gradual release over time
    Cliff,     // All tokens at cliff time
    Hybrid,    // Cliff amount + linear remainder
}
```

### VestingSchedule
```rust
struct VestingSchedule {
    beneficiary: Address,
    token: Address,
    amount: i128,
    claimed: i128,
    vesting_type: VestingType,
    start_time: u64,
    cliff_time: u64,
    end_time: u64,
    cliff_amount: Option<i128>,
    revocable: bool,
    revoked: bool,
}
```

### AirdropType
```rust
enum AirdropType {
    Equal,     // Same amount to all
    Weighted,  // Custom amounts
    Claimable, // Recipients claim individually
}
```

### AirdropCampaign
```rust
struct AirdropCampaign {
    token: Address,
    airdrop_type: AirdropType,
    total_amount: i128,
    distributed: i128,
    start_time: u64,
    end_time: u64,
    active: bool,
}
```

### Launch
```rust
struct Launch {
    creator: Address,
    token: Address,
    name: String,
    symbol: String,
    total_supply: i128,
    vesting_contract: Option<Address>,
    airdrop_contract: Option<Address>,
    website: String,
    description: String,
    created_at: u64,
    active: bool,
}
```

## Error Handling

All contract functions use Rust's `panic!` macro for error conditions:

- **"Unauthorized"**: Caller lacks required permissions
- **"Invalid parameters"**: Input validation failed  
- **"Contract not initialized"**: Contract initialization required
- **"Already initialized"**: Contract already initialized
- **"Invalid state"**: Operation not allowed in current state
- **"Insufficient balance"**: Not enough tokens for operation
- **"Schedule not found"**: Vesting schedule doesn't exist
- **"Campaign not found"**: Airdrop campaign doesn't exist
- **"Already claimed"**: Tokens already claimed for this recipient

## Usage Examples

See individual contract documentation files for detailed usage examples:
- [Token Contract](token.md)
- [Vesting Contract](vesting.md)
- [Airdrop Contract](airdrop.md)
- [Launchpad Contract](launchpad.md)