# Token Contract

The Token Contract is a SAC-compatible fungible token with advanced launch configuration features including minting, burning, pausing, and admin controls.

## Features

- **ERC20-compatible**: Standard transfer, approve, and allowance functionality
- **Mintable**: Optional minting capability controlled by admin
- **Burnable**: Optional burning capability 
- **Pausable**: Emergency pause/unpause functionality
- **Admin Controls**: Configurable admin with ability to transfer ownership
- **Overflow Protection**: Safe arithmetic operations with overflow checks

## Contract Interface

### Initialization

```rust
pub fn initialize(
    env: Env,
    admin: Address,
    name: String,
    symbol: String,
    decimals: u32,
    total_supply: i128,
    mintable: bool,
    burnable: bool,
)
```

Initializes the token contract with configuration parameters. Can only be called once.

**Parameters:**
- `admin`: The administrative address with special privileges
- `name`: Human-readable name of the token
- `symbol`: Short symbol/ticker for the token
- `decimals`: Number of decimal places (max 18)
- `total_supply`: Initial total supply minted to admin
- `mintable`: Whether the token can be minted after initialization
- `burnable`: Whether token holders can burn their tokens

### Core Token Functions

#### Transfer Functions

```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128)
pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128)
pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32)
```

Standard ERC20-style transfer and approval functionality.

#### Balance Queries

```rust
pub fn balance(env: Env, id: Address) -> i128
pub fn allowance(env: Env, from: Address, spender: Address) -> i128
pub fn total_supply(env: Env) -> i128
```

Query functions for balances and allowances.

#### Token Metadata

```rust
pub fn name(env: Env) -> String
pub fn symbol(env: Env) -> String  
pub fn decimals(env: Env) -> u32
pub fn get_config(env: Env) -> TokenConfig
```

Functions to retrieve token metadata and configuration.

### Administrative Functions

#### Minting and Burning

```rust
pub fn mint(env: Env, admin: Address, to: Address, amount: i128)
pub fn burn(env: Env, from: Address, amount: i128)
```

**Mint**: Only admin can mint tokens (if mintable is enabled)
**Burn**: Token holders can burn their own tokens (if burnable is enabled)

#### Pause Controls

```rust
pub fn pause(env: Env, admin: Address)
pub fn unpause(env: Env, admin: Address)
```

Admin can pause/unpause the contract, preventing all transfers when paused.

#### Admin Management

```rust
pub fn set_admin(env: Env, admin: Address, new_admin: Address)
```

Current admin can transfer administrative privileges to a new address.

## Security Features

### Access Control
- All admin functions require authentication from the current admin address
- Transfers require authentication from the sender address
- Allowance-based transfers require authentication from the spender

### Input Validation
- Prevents initialization of already initialized contracts
- Validates positive amounts for transfers and mints
- Checks sufficient balances and allowances before transfers
- Enforces maximum decimal places (18)
- Validates future expiration dates for approvals

### State Protection  
- Overflow protection on all arithmetic operations
- Atomic state updates to prevent inconsistencies
- Proper storage separation (persistent for balances, temporary for allowances)

## Error Handling

The contract uses descriptive panic messages for error conditions:

- `"Already initialized"` - Contract already initialized
- `"Not initialized"` - Contract not yet initialized
- `"Unauthorized"` - Caller lacks required permissions
- `"Token not mintable"` - Attempt to mint when minting disabled
- `"Token not burnable"` - Attempt to burn when burning disabled  
- `"Token is paused"` - Operation attempted while contract paused
- `"Insufficient balance"` - Transfer amount exceeds balance
- `"Insufficient allowance"` - Transfer amount exceeds allowance
- `"Amount must be positive"` - Invalid zero or negative amount
- `"Balance overflow"` - Arithmetic overflow in balance calculation
- `"Total supply overflow"` - Arithmetic overflow in total supply

## Usage Examples

### Deploy and Initialize Token

```rust
// Initialize a mintable, burnable token
client.initialize(
    &admin_address,
    &String::from_str(&env, "My Token"),
    &String::from_str(&env, "MTK"), 
    &7u32,           // 7 decimals
    &1_000_000i128,  // 1M initial supply
    &true,           // mintable
    &true            // burnable
);
```

### Mint Additional Tokens

```rust
// Admin mints 500K tokens to user
client.mint(&admin_address, &user_address, &500_000i128);
```

### Transfer Tokens

```rust
// Direct transfer
client.transfer(&from_address, &to_address, &100_000i128);

// Allowance-based transfer
client.approve(&owner_address, &spender_address, &200_000i128, &expiration_ledger);
client.transfer_from(&spender_address, &owner_address, &to_address, &150_000i128);
```

### Emergency Pause

```rust
// Pause all transfers
client.pause(&admin_address);

// Resume operations  
client.unpause(&admin_address);
```

## Integration with Launchpad

The Token Contract is designed to work seamlessly with other Stellar Launchpad contracts:

- **Vesting Contract**: Can hold and release tokens according to vesting schedules
- **Airdrop Contract**: Can distribute tokens to multiple recipients efficiently  
- **Launchpad Registry**: Tracks token launches and associated contracts

The admin should be set to the Launchpad Registry contract address to enable coordinated management across the ecosystem.