# Airdrop Contract

The Airdrop Contract enables efficient distribution of tokens to multiple recipients through configurable airdrop campaigns.

## Features

- **Multiple Distribution Types**: Equal, Weighted, and Claimable airdrops
- **Flexible Scheduling**: Configure start and end ledger sequences
- **Batch Operations**: Add multiple recipients and distribute tokens efficiently
- **Individual Claims**: Recipients can claim their own allocations in claimable campaigns
- **Admin Controls**: Full campaign management and cancellation capabilities

## Contract Structure

### AirdropType
```rust
pub enum AirdropType {
    Equal,      // Same amount to all recipients
    Weighted,   // Different amounts per recipient
    Claimable,  // Recipients claim their own allocation
}
```

### AirdropCampaign
```rust
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
```

### RecipientAllocation
```rust
pub struct RecipientAllocation {
    pub recipient: Address,
    pub amount: i128,
    pub claimed: bool,
}
```

## Functions

### Administrative Functions

#### `initialize(env: Env, admin: Address)`
Initializes the airdrop contract with an admin address.

**Parameters:**
- `admin`: Address of the contract administrator

**Authorization:** Requires admin signature

#### `create_campaign(...) -> u64`
Creates a new airdrop campaign.

**Parameters:**
- `admin`: Campaign administrator
- `token`: Token contract address to distribute
- `airdrop_type`: Distribution type (Equal, Weighted, or Claimable)
- `total_amount`: Total tokens to distribute
- `start_ledger`: Campaign start ledger sequence
- `end_ledger`: Campaign end ledger sequence

**Returns:** Campaign ID

**Authorization:** Requires admin signature

#### `add_recipients(env: Env, admin: Address, campaign_id: u64, recipients: Vec<Address>, amounts: Vec<i128>)`
Adds recipients and their allocations to a campaign.

**Parameters:**
- `admin`: Campaign administrator
- `campaign_id`: Target campaign ID
- `recipients`: List of recipient addresses
- `amounts`: Corresponding allocation amounts

**Authorization:** Requires admin signature

**Note:** Can only be called before campaign starts

#### `cancel_campaign(env: Env, admin: Address, campaign_id: u64)`
Cancels an active campaign.

**Parameters:**
- `admin`: Campaign administrator
- `campaign_id`: Campaign to cancel

**Authorization:** Requires admin signature

### Distribution Functions

#### `distribute(env: Env, admin: Address, campaign_id: u64)`
Distributes tokens to all recipients in Equal or Weighted campaigns.

**Parameters:**
- `admin`: Campaign administrator
- `campaign_id`: Campaign to distribute

**Authorization:** Requires admin signature

**Note:** Not available for Claimable campaigns

#### `claim(env: Env, campaign_id: u64, recipient: Address)`
Allows recipients to claim their allocation in Claimable campaigns.

**Parameters:**
- `campaign_id`: Campaign to claim from
- `recipient`: Address claiming tokens

**Authorization:** Requires recipient signature

**Note:** Only available for Claimable campaigns

### Query Functions

#### `get_campaign(env: Env, campaign_id: u64) -> AirdropCampaign`
Returns complete campaign information.

#### `get_allocation(env: Env, campaign_id: u64, recipient: Address) -> i128`
Returns allocation amount for a specific recipient.

#### `get_campaigns_by_admin(env: Env, admin: Address) -> Vec<u64>`
Returns list of campaign IDs created by an admin.

## Usage Examples

### Creating an Equal Distribution Campaign
```rust
// All recipients receive the same amount
let campaign_id = client.create_campaign(
    &admin,
    &token_address,
    &AirdropType::Equal,
    &1_000_000i128, // 1M tokens total
    &current_ledger + 100,
    &current_ledger + 1000,
);

// Add recipients (amounts will be divided equally)
let recipients = vec![user1, user2, user3];
let amounts = vec![333_333i128, 333_333i128, 333_334i128]; // Equal distribution
client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

// Distribute all at once
client.distribute(&admin, &campaign_id);
```

### Creating a Weighted Distribution Campaign
```rust
// Different amounts per recipient
let campaign_id = client.create_campaign(
    &admin,
    &token_address,
    &AirdropType::Weighted,
    &1_000_000i128,
    &current_ledger + 100,
    &current_ledger + 1000,
);

// Add recipients with different amounts
let recipients = vec![whale, dolphin, shrimp];
let amounts = vec![500_000i128, 300_000i128, 200_000i128];
client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

// Distribute all at once
client.distribute(&admin, &campaign_id);
```

### Creating a Claimable Campaign
```rust
// Recipients claim individually
let campaign_id = client.create_campaign(
    &admin,
    &token_address,
    &AirdropType::Claimable,
    &1_000_000i128,
    &current_ledger + 100,
    &current_ledger + 1000,
);

// Add recipients
let recipients = vec![user1, user2, user3];
let amounts = vec![400_000i128, 300_000i128, 300_000i128];
client.add_recipients(&admin, &campaign_id, &recipients, &amounts);

// Recipients claim individually
client.claim(&campaign_id, &user1);
client.claim(&campaign_id, &user2);
// user3 can claim later
```

## Security Considerations

1. **Authorization**: All admin functions require proper authentication
2. **Timing Controls**: Campaigns respect start/end ledger boundaries
3. **Double Claiming**: Prevented through claimed status tracking
4. **Amount Validation**: Total allocations cannot exceed campaign amount
5. **State Management**: Proper campaign state transitions (active/inactive)

## Error Handling

The contract includes comprehensive error handling for:
- Unauthorized access attempts
- Invalid timing parameters
- Exceeding allocation limits
- Double claiming attempts
- Invalid campaign states
- Missing campaigns or allocations

## Testing

The contract includes 12 comprehensive tests covering:
- Contract initialization
- Campaign creation and management
- Recipient addition and validation
- Distribution mechanisms
- Claiming functionality
- Error conditions and edge cases
- Authorization requirements