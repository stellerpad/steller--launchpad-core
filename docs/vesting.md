# Vesting Contract

The Vesting Contract enables token release scheduling with multiple vesting strategies including linear, cliff, and hybrid vesting mechanisms. It allows for sophisticated token distribution over time with administrative controls.

## Features

- **Multiple Vesting Types**: Linear, cliff, and hybrid vesting strategies
- **Flexible Scheduling**: Configurable start, cliff, and end ledgers
- **Administrative Controls**: Admin can create and optionally revoke schedules
- **Beneficiary Queries**: Track vesting schedules by beneficiary or token
- **Claimable Calculation**: Real-time calculation of available tokens
- **Revocation Support**: Optional revocable schedules for enhanced control

## Vesting Types

### Linear Vesting
Tokens are released gradually over time from start to end ledger.
- **Use Case**: Gradual employee compensation, contributor rewards
- **Release Pattern**: Smooth, continuous release
- **Cliff Amount**: Must be 0

### Cliff Vesting  
All tokens are released at once after the cliff period.
- **Use Case**: Performance bonuses, milestone payments
- **Release Pattern**: All-or-nothing at cliff
- **Cliff Amount**: Must equal total amount

### Hybrid Vesting
Combines cliff and linear vesting - cliff amount released immediately at cliff, remainder released linearly.
- **Use Case**: Sign-on bonuses with ongoing vesting, advisory agreements
- **Release Pattern**: Cliff portion + linear remainder
- **Cliff Amount**: Between 0 and total amount (exclusive)

## Contract Interface

### Initialization

```rust
pub fn initialize(env: Env, admin: Address)
```

Initializes the vesting contract with an administrative address. Can only be called once.

### Schedule Management

#### Create Schedule

```rust
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
) -> u64
```

Creates a new vesting schedule and returns the schedule ID.

**Parameters:**
- `admin`: Must match contract admin for authorization
- `beneficiary`: Address that can claim vested tokens
- `token`: Token contract address
- `total_amount`: Total tokens to vest
- `start_ledger`: When vesting begins
- `cliff_ledger`: When cliff vesting occurs (between start and end)
- `end_ledger`: When vesting completes
- `vesting_type`: Linear, Cliff, or Hybrid
- `cliff_amount`: Amount for cliff (must match vesting type rules)
- `revocable`: Whether admin can revoke this schedule

#### Revoke Schedule

```rust
pub fn revoke(env: Env, admin: Address, schedule_id: u64)
```

Revokes a vesting schedule (if revocable). After revocation, no more tokens can be claimed.

### Token Operations

#### Release Tokens

```rust
pub fn release(env: Env, schedule_id: u64)
```

Releases all currently claimable tokens from a schedule to the beneficiary.

#### Check Claimable Amount

```rust
pub fn claimable_amount(env: Env, schedule_id: u64) -> i128
```

Returns the amount of tokens currently available to claim from a schedule.

### Query Functions

#### Get Schedule

```rust
pub fn get_schedule(env: Env, schedule_id: u64) -> VestingSchedule
```

Returns complete details for a specific vesting schedule.

#### Get Schedules by Beneficiary

```rust
pub fn get_schedules_by_beneficiary(env: Env, beneficiary: Address) -> Vec<VestingSchedule>
```

Returns all vesting schedules for a specific beneficiary.

#### Get Schedules by Token

```rust
pub fn get_schedules_by_token(env: Env, token: Address) -> Vec<VestingSchedule>
```

Returns all vesting schedules for a specific token.

## Vesting Schedule Structure

```rust
pub struct VestingSchedule {
    pub beneficiary: Address,    // Who can claim tokens
    pub token: Address,          // Token being vested
    pub total_amount: i128,      // Total tokens to vest
    pub start_ledger: u32,       // Vesting start time
    pub cliff_ledger: u32,       // Cliff time
    pub end_ledger: u32,         // Vesting end time
    pub vesting_type: VestingType, // Vesting strategy
    pub cliff_amount: i128,      // Cliff portion
    pub released: i128,          // Already released amount
    pub revocable: bool,         // Can be revoked by admin
    pub revoked: bool,           // Has been revoked
}
```

## Vesting Calculations

### Linear Vesting Formula
```
vested_amount = (total_amount * elapsed_time) / total_duration
claimable = vested_amount - already_released
```

### Cliff Vesting Formula
```
vested_amount = current_ledger >= cliff_ledger ? total_amount : 0
claimable = vested_amount - already_released
```

### Hybrid Vesting Formula
```
cliff_vested = current_ledger >= cliff_ledger ? cliff_amount : 0
linear_vested = linear portion calculated from cliff_ledger to end_ledger
total_vested = cliff_vested + linear_vested
claimable = total_vested - already_released
```

## Security Features

### Access Control
- Only admin can create and revoke schedules
- Only beneficiary can release their vested tokens
- Admin authorization required for all administrative functions

### Input Validation
- Prevents double initialization
- Validates ledger sequence (start < cliff <= end)
- Enforces vesting type rules for cliff amounts
- Checks positive amounts and valid addresses

### State Protection
- Immutable schedule parameters after creation
- Atomic token releases prevent double-spending
- Revocation permanently disables schedules
- Safe arithmetic with overflow protection

## Error Handling

The contract uses descriptive panic messages:

- `"Already initialized"` - Contract already initialized
- `"Not initialized"` - Contract not yet initialized  
- `"Unauthorized"` - Caller lacks admin privileges
- `"Schedule not found"` - Invalid schedule ID
- `"Schedule revoked"` - Operation on revoked schedule
- `"Schedule not revocable"` - Attempt to revoke non-revocable schedule
- `"Already revoked"` - Schedule already revoked
- `"Amount must be positive"` - Invalid zero/negative amount
- `"Start must be before end"` - Invalid ledger sequence
- `"Invalid cliff ledger"` - Cliff outside start-end range
- `"No tokens to release"` - Nothing currently claimable

## Usage Examples

### Linear Vesting (Employee Compensation)

```rust
// 4-year linear vesting starting immediately
let schedule_id = client.create_schedule(
    &admin,
    &employee,
    &token_address,
    &4_000_000i128,    // 4M tokens
    &current_ledger,   // start now  
    &current_ledger,   // no cliff for linear
    &(current_ledger + FOUR_YEARS), // end in 4 years
    &VestingType::Linear,
    &0i128,            // no cliff amount
    &true              // revocable
);
```

### Cliff Vesting (Performance Bonus)

```rust
// All tokens released after 1 year cliff
let schedule_id = client.create_schedule(
    &admin,
    &employee,
    &token_address, 
    &1_000_000i128,    // 1M token bonus
    &current_ledger,   // start now
    &(current_ledger + ONE_YEAR), // cliff after 1 year
    &(current_ledger + ONE_YEAR), // end same as cliff
    &VestingType::Cliff,
    &1_000_000i128,    // cliff = total for cliff vesting
    &false             // non-revocable
);
```

### Hybrid Vesting (Advisor Agreement)

```rust
// 25% immediately, 75% over 2 years
let total = 2_000_000i128;
let cliff_portion = total / 4; // 25%

let schedule_id = client.create_schedule(
    &admin,
    &advisor,
    &token_address,
    &total,
    &current_ledger,   // start now
    &current_ledger,   // immediate cliff
    &(current_ledger + TWO_YEARS), // linear over 2 years
    &VestingType::Hybrid,
    &cliff_portion,    // 25% immediately
    &true              // revocable
);
```

### Check and Release Tokens

```rust
// Check what's available
let claimable = client.claimable_amount(&schedule_id);

// Release available tokens
if claimable > 0 {
    client.release(&schedule_id);
}

// Check schedule status
let schedule = client.get_schedule(&schedule_id);
println!("Released: {}/{}", schedule.released, schedule.total_amount);
```

## Integration with Launchpad

The Vesting Contract integrates with other Stellar Launchpad components:

- **Token Contract**: Manages the actual token transfers during releases
- **Launchpad Registry**: Tracks which tokens have associated vesting schedules  
- **Airdrop Contract**: Can create vesting schedules for airdrop recipients

The contract is designed to work with any SAC-compatible token and can be used for various distribution strategies across the token launch lifecycle.