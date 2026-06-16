# Launchpad Registry Contract

The Launchpad Registry Contract serves as the central registry for all token launches on the platform, tracking launch metadata and coordinating with vesting and airdrop contracts.

## Features

- **Launch Registration**: Register new token launches with comprehensive metadata
- **Creator Management**: Track launches by creator with full history
- **Contract Integration**: Link vesting and airdrop contracts to launches
- **Admin Controls**: Administrative oversight and launch deactivation
- **Query Functions**: Comprehensive launch discovery and filtering

## Contract Structure

### TokenLaunch
```rust
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
```

## Functions

### Administrative Functions

#### `initialize(env: Env, admin: Address)`
Initializes the launchpad registry with an admin address.

**Parameters:**
- `admin`: Address of the contract administrator

**Authorization:** Requires admin signature

#### `deactivate_launch(env: Env, admin: Address, launch_id: u64)`
Deactivates a specific launch (admin only).

**Parameters:**
- `admin`: Administrator address
- `launch_id`: Launch to deactivate

**Authorization:** Requires admin signature

### Launch Management Functions

#### `register_launch(...) -> u64`
Registers a new token launch in the registry.

**Parameters:**
- `creator`: Launch creator address
- `token_contract`: Address of the token contract
- `name`: Token name
- `symbol`: Token symbol
- `total_supply`: Total token supply
- `vesting_contract`: Optional vesting contract address
- `airdrop_contract`: Optional airdrop contract address
- `website`: Project website URL
- `description`: Project description

**Returns:** Launch ID

**Authorization:** Requires creator signature

#### `update_launch_contracts(env: Env, creator: Address, launch_id: u64, vesting_contract: Option<Address>, airdrop_contract: Option<Address>)`
Updates vesting and airdrop contract addresses for an existing launch.

**Parameters:**
- `creator`: Launch creator (must own the launch)
- `launch_id`: Launch to update
- `vesting_contract`: New vesting contract address
- `airdrop_contract`: New airdrop contract address

**Authorization:** Requires creator signature

### Query Functions

#### `get_launch(env: Env, launch_id: u64) -> TokenLaunch`
Returns complete launch information by ID.

#### `get_launches_by_creator(env: Env, creator: Address) -> Vec<TokenLaunch>`
Returns all launches created by a specific address.

#### `get_all_launches(env: Env) -> Vec<TokenLaunch>`
Returns all launches in the registry (active and inactive).

#### `get_active_launches(env: Env) -> Vec<TokenLaunch>`
Returns only active launches.

#### `get_total_launches(env: Env) -> u64`
Returns the total number of launches ever registered.

## Usage Examples

### Registering a New Token Launch
```rust
let launch_id = client.register_launch(
    &creator,
    &token_contract,
    &String::from_str(&env, "MyToken"),
    &String::from_str(&env, "MTK"),
    &1_000_000_000i128, // 1 billion tokens
    &None, // No vesting contract yet
    &None, // No airdrop contract yet
    &String::from_str(&env, "https://mytoken.com"),
    &String::from_str(&env, "Revolutionary DeFi token for the future"),
);
```

### Adding Vesting and Airdrop Contracts Later
```rust
// After deploying vesting and airdrop contracts
client.update_launch_contracts(
    &creator,
    &launch_id,
    &Some(vesting_contract_address),
    &Some(airdrop_contract_address),
);
```

### Discovering Launches
```rust
// Get all launches by a specific creator
let creator_launches = client.get_launches_by_creator(&creator_address);

// Get all active launches
let active_launches = client.get_active_launches();

// Get launch statistics
let total_count = client.get_total_launches();

// Get specific launch details
let launch = client.get_launch(&launch_id);
```

### Launch Lifecycle Management
```rust
// 1. Register initial launch
let launch_id = client.register_launch(
    &creator,
    &token_address,
    &name,
    &symbol,
    &supply,
    &None, // Will add later
    &None, // Will add later
    &website,
    &description,
);

// 2. Deploy and link vesting contract
let vesting_address = deploy_vesting_contract(&env, &creator, &token_address);
client.update_launch_contracts(&creator, &launch_id, &Some(vesting_address), &None);

// 3. Deploy and link airdrop contract
let airdrop_address = deploy_airdrop_contract(&env, &creator, &token_address);
client.update_launch_contracts(&creator, &launch_id, &Some(vesting_address), &Some(airdrop_address));

// 4. Admin can deactivate if needed
// client.deactivate_launch(&admin, &launch_id);
```

## Integration with Other Contracts

### Token Contract Integration
The registry stores the token contract address and tracks key metadata like name, symbol, and total supply for discovery purposes.

### Vesting Contract Integration
Links to vesting contracts enable users to:
- Discover which launches have vesting schedules
- Navigate directly to vesting functionality
- Track tokenomics distribution

### Airdrop Contract Integration  
Links to airdrop contracts enable users to:
- Find active airdrop campaigns
- Participate in token distribution events
- Track community engagement

## Security Considerations

1. **Creator Authorization**: Only launch creators can update their own launches
2. **Admin Controls**: Admins can deactivate launches but cannot modify creator data
3. **Immutable Core Data**: Critical launch data like token contract and supply cannot be changed after registration
4. **Optional Contracts**: Vesting and airdrop contracts are optional and updatable
5. **Active Status**: Inactive launches remain queryable but are excluded from active listings

## Registry Benefits

1. **Discovery**: Central place to find all launched tokens
2. **Verification**: Links between tokens and their associated contracts
3. **History**: Complete launch history and creator tracking  
4. **Integration**: Seamless connection to vesting and airdrop functionality
5. **Administration**: Platform oversight and quality control

## Error Handling

The contract includes comprehensive error handling for:
- Uninitialized registry access
- Invalid launch parameters (empty names, zero supply)
- Unauthorized modification attempts
- Missing launch lookups
- Duplicate initialization attempts

## Testing

The contract includes 10 comprehensive tests covering:
- Registry initialization and configuration
- Launch registration with validation
- Creator-based launch queries
- Administrative deactivation
- Contract address updates
- Active/inactive launch filtering
- Authorization requirements
- Error conditions and edge cases