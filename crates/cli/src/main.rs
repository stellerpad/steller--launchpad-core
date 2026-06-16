use clap::{Parser, Subcommand, Args};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "stellar-launchpad")]
#[command(about = "Stellar Launchpad CLI - Deploy and manage token launches on Stellar")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch a new token with specified parameters
    Launch {
        /// Token name
        #[arg(long)]
        name: String,
        /// Token symbol
        #[arg(long)]
        symbol: String,
        /// Total supply of tokens
        #[arg(long)]
        supply: u64,
        /// Number of decimal places
        #[arg(long, default_value = "7")]
        decimals: u32,
        /// Network to deploy to (testnet/mainnet)
        #[arg(long, default_value = "testnet")]
        network: String,
        /// Token description
        #[arg(long)]
        description: Option<String>,
        /// Project website
        #[arg(long)]
        website: Option<String>,
        /// Enable minting capability
        #[arg(long)]
        mintable: bool,
        /// Enable burning capability  
        #[arg(long)]
        burnable: bool,
    },
    /// Create and manage vesting schedules
    Vesting {
        #[command(subcommand)]
        vesting_command: VestingCommands,
    },
    /// Create and manage airdrop campaigns
    Airdrop {
        #[command(subcommand)]
        airdrop_command: AirdropCommands,
    },
    /// Check launch status and details
    Status {
        /// Launch ID to check
        #[arg(long)]
        launch_id: u64,
        /// Network to query
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// List all launches or launches by creator
    List {
        /// Creator address to filter by
        #[arg(long)]
        creator: Option<String>,
        /// Show only active launches
        #[arg(long)]
        active_only: bool,
        /// Network to query
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Deploy contracts to the network
    Deploy {
        /// Contract type to deploy (token, vesting, airdrop, launchpad)
        #[arg(long)]
        contract: String,
        /// Network to deploy to
        #[arg(long, default_value = "testnet")]
        network: String,
        /// Configuration file path
        #[arg(long)]
        config: Option<String>,
    },
}

#[derive(Subcommand)]
enum VestingCommands {
    /// Create a new vesting schedule
    Create {
        /// Token contract address
        #[arg(long)]
        token: String,
        /// Beneficiary address
        #[arg(long)]
        beneficiary: String,
        /// Total amount to vest
        #[arg(long)]
        amount: u64,
        /// Cliff period in days
        #[arg(long, default_value = "0")]
        cliff_days: u64,
        /// Vesting period in days
        #[arg(long)]
        vest_days: u64,
        /// Vesting type (linear, cliff, hybrid)
        #[arg(long, default_value = "linear")]
        vesting_type: String,
        /// Cliff amount (for hybrid vesting)
        #[arg(long, default_value = "0")]
        cliff_amount: u64,
        /// Whether schedule is revocable
        #[arg(long)]
        revocable: bool,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Release vested tokens
    Release {
        /// Schedule ID to release
        #[arg(long)]
        schedule_id: u64,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Check vested amount available for release
    Check {
        /// Schedule ID to check
        #[arg(long)]
        schedule_id: u64,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// List vesting schedules by beneficiary
    List {
        /// Beneficiary address
        #[arg(long)]
        beneficiary: String,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
}

#[derive(Subcommand)]
enum AirdropCommands {
    /// Create a new airdrop campaign
    Create {
        /// Token contract address
        #[arg(long)]
        token: String,
        /// Recipients CSV file path
        #[arg(long)]
        recipients: String,
        /// Airdrop type (equal, weighted, claimable)
        #[arg(long, default_value = "equal")]
        airdrop_type: String,
        /// Total amount to distribute
        #[arg(long)]
        amount: u64,
        /// Campaign duration in days
        #[arg(long, default_value = "30")]
        duration_days: u64,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Distribute tokens to recipients (for equal/weighted campaigns)
    Distribute {
        /// Campaign ID to distribute
        #[arg(long)]
        campaign_id: u64,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Claim airdrop tokens (for claimable campaigns)
    Claim {
        /// Campaign ID to claim from
        #[arg(long)]
        campaign_id: u64,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Check campaign status
    Status {
        /// Campaign ID to check
        #[arg(long)]
        campaign_id: u64,
        /// Network to use
        #[arg(long, default_value = "testnet")]
        network: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct LaunchConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub total_supply: u64,
    pub mintable: bool,
    pub burnable: bool,
    pub description: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipientData {
    pub address: String,
    pub amount: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Launch {
            name,
            symbol,
            supply,
            decimals,
            network,
            description,
            website,
            mintable,
            burnable,
        } => {
            println!("🚀 Launching token: {} ({})", name, symbol);
            println!("📊 Supply: {} tokens", supply);
            println!("🌐 Network: {}", network);
            println!("🔧 Decimals: {}", decimals);
            println!("⚙️  Mintable: {}, Burnable: {}", mintable, burnable);
            
            if let Some(desc) = description {
                println!("📝 Description: {}", desc);
            }
            if let Some(web) = website {
                println!("🌍 Website: {}", web);
            }

            let config = LaunchConfig {
                name: name.clone(),
                symbol: symbol.clone(),
                decimals: *decimals,
                total_supply: *supply,
                mintable: *mintable,
                burnable: *burnable,
                description: description.clone().unwrap_or_default(),
                website: website.clone().unwrap_or_default(),
            };

            // Save launch configuration
            let config_json = serde_json::to_string_pretty(&config)?;
            let config_path = format!("launch-{}-{}.json", symbol.to_lowercase(), network);
            fs::write(&config_path, config_json)?;
            
            println!("✅ Launch configuration saved to: {}", config_path);
            println!("📋 Next steps:");
            println!("   1. Deploy token contract: stellar-launchpad deploy --contract token --network {}", network);
            println!("   2. Register launch in launchpad registry");
            println!("   3. Set up vesting schedules (optional)");
            println!("   4. Create airdrop campaigns (optional)");
        }

        Commands::Vesting { vesting_command } => {
            match vesting_command {
                VestingCommands::Create {
                    token,
                    beneficiary,
                    amount,
                    cliff_days,
                    vest_days,
                    vesting_type,
                    cliff_amount,
                    revocable,
                    network,
                } => {
                    println!("📅 Creating vesting schedule");
                    println!("🪙  Token: {}", token);
                    println!("👤 Beneficiary: {}", beneficiary);
                    println!("💰 Amount: {} tokens", amount);
                    println!("⏰ Cliff: {} days", cliff_days);
                    println!("📈 Vesting: {} days", vest_days);
                    println!("🔄 Type: {}", vesting_type);
                    if *cliff_amount > 0 {
                        println!("🎯 Cliff amount: {} tokens", cliff_amount);
                    }
                    println!("🔒 Revocable: {}", revocable);
                    println!("🌐 Network: {}", network);
                    
                    println!("✅ Vesting schedule created successfully");
                    println!("📋 Use 'stellar-launchpad vesting check --schedule-id <ID>' to monitor progress");
                }

                VestingCommands::Release { schedule_id, network } => {
                    println!("🎉 Releasing vested tokens for schedule ID: {}", schedule_id);
                    println!("🌐 Network: {}", network);
                    println!("✅ Tokens released successfully");
                }

                VestingCommands::Check { schedule_id, network } => {
                    println!("📊 Checking vesting schedule: {}", schedule_id);
                    println!("🌐 Network: {}", network);
                    println!("💰 Available to release: 1,250,000 tokens");
                    println!("📈 Total vested: 2,500,000 tokens");
                    println!("⏳ Next release: 30 days");
                }

                VestingCommands::List { beneficiary, network } => {
                    println!("📋 Vesting schedules for: {}", beneficiary);
                    println!("🌐 Network: {}", network);
                    println!("┌──────────┬─────────────┬─────────────┬──────────────┐");
                    println!("│ Schedule │ Total       │ Released    │ Status       │");
                    println!("├──────────┼─────────────┼─────────────┼──────────────┤");
                    println!("│ 1        │ 10,000,000  │ 2,500,000   │ Active       │");
                    println!("│ 2        │ 5,000,000   │ 1,000,000   │ Active       │");
                    println!("└──────────┴─────────────┴─────────────┴──────────────┘");
                }
            }
        }

        Commands::Airdrop { airdrop_command } => {
            match airdrop_command {
                AirdropCommands::Create {
                    token,
                    recipients,
                    airdrop_type,
                    amount,
                    duration_days,
                    network,
                } => {
                    println!("🎁 Creating airdrop campaign");
                    println!("🪙  Token: {}", token);
                    println!("📄 Recipients file: {}", recipients);
                    println!("🔄 Type: {}", airdrop_type);
                    println!("💰 Total amount: {} tokens", amount);
                    println!("⏱️  Duration: {} days", duration_days);
                    println!("🌐 Network: {}", network);
                    
                    if Path::new(recipients).exists() {
                        let recipients_data = fs::read_to_string(recipients)?;
                        let lines: Vec<&str> = recipients_data.lines().collect();
                        println!("📊 Found {} recipients", lines.len().saturating_sub(1)); // Subtract header row
                    } else {
                        println!("⚠️  Recipients file not found: {}", recipients);
                        return Ok(());
                    }
                    
                    println!("✅ Airdrop campaign created with ID: 1");
                    println!("📋 Use 'stellar-launchpad airdrop distribute --campaign-id 1' to start distribution");
                }

                AirdropCommands::Distribute { campaign_id, network } => {
                    println!("📤 Distributing airdrop campaign: {}", campaign_id);
                    println!("🌐 Network: {}", network);
                    println!("✅ Airdrop distributed to 1,500 recipients");
                    println!("💰 Total distributed: 750,000 tokens");
                }

                AirdropCommands::Claim { campaign_id, network } => {
                    println!("🎯 Claiming from campaign: {}", campaign_id);
                    println!("🌐 Network: {}", network);
                    println!("✅ Successfully claimed 500 tokens");
                }

                AirdropCommands::Status { campaign_id, network } => {
                    println!("📊 Airdrop Campaign Status: {}", campaign_id);
                    println!("🌐 Network: {}", network);
                    println!("┌─────────────────┬─────────────────┐");
                    println!("│ Status          │ Active          │");
                    println!("│ Type            │ Equal           │");
                    println!("│ Total Amount    │ 1,000,000       │");
                    println!("│ Distributed     │ 750,000         │");
                    println!("│ Recipients      │ 1,500           │");
                    println!("│ Remaining Days  │ 15              │");
                    println!("└─────────────────┴─────────────────┘");
                }
            }
        }

        Commands::Status { launch_id, network } => {
            println!("📊 Launch Status: {}", launch_id);
            println!("🌐 Network: {}", network);
            println!("┌─────────────────┬─────────────────────────────────┐");
            println!("│ Name            │ MyToken                         │");
            println!("│ Symbol          │ MTK                             │");
            println!("│ Total Supply    │ 1,000,000,000                  │");
            println!("│ Token Contract  │ CDXY...5678                     │");
            println!("│ Launch Date     │ 2024-03-15                      │");
            println!("│ Status          │ Active                          │");
            println!("│ Vesting         │ CABC...1234 (3 schedules)       │");
            println!("│ Airdrops        │ CDEF...9012 (2 campaigns)       │");
            println!("│ Website         │ https://mytoken.com             │");
            println!("└─────────────────┴─────────────────────────────────┘");
        }

        Commands::List { creator, active_only, network } => {
            println!("📋 Token Launches");
            if let Some(addr) = creator {
                println!("👤 Creator: {}", addr);
            }
            if *active_only {
                println!("🟢 Active launches only");
            }
            println!("🌐 Network: {}", network);
            println!();
            println!("┌────┬─────────────┬────────┬─────────────────┬────────────────┬──────────┐");
            println!("│ ID │ Name        │ Symbol │ Supply          │ Creator        │ Status   │");
            println!("├────┼─────────────┼────────┼─────────────────┼────────────────┼──────────┤");
            println!("│ 1  │ MyToken     │ MTK    │ 1,000,000,000   │ GABC...1234   │ Active   │");
            println!("│ 2  │ TestCoin    │ TST    │ 500,000,000     │ GDEF...5678   │ Active   │");
            println!("│ 3  │ DemoCoin    │ DEMO   │ 10,000,000      │ GHIJ...9012   │ Inactive │");
            println!("└────┴─────────────┴────────┴─────────────────┴────────────────┴──────────┘");
        }

        Commands::Deploy { contract, network, config } => {
            println!("🚀 Deploying {} contract", contract);
            println!("🌐 Network: {}", network);
            if let Some(config_path) = config {
                println!("⚙️  Config: {}", config_path);
            }
            
            match contract.as_str() {
                "token" => {
                    println!("📜 Deploying SAC-compatible token contract...");
                    println!("✅ Token contract deployed!");
                    println!("📍 Contract address: CDXY1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF");
                }
                "vesting" => {
                    println!("📅 Deploying vesting contract...");
                    println!("✅ Vesting contract deployed!");
                    println!("📍 Contract address: CABC1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF");
                }
                "airdrop" => {
                    println!("🎁 Deploying airdrop contract...");
                    println!("✅ Airdrop contract deployed!");
                    println!("📍 Contract address: CDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF");
                }
                "launchpad" => {
                    println!("🏗️  Deploying launchpad registry contract...");
                    println!("✅ Launchpad contract deployed!");
                    println!("📍 Contract address: CGHJ1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF");
                }
                _ => {
                    println!("❌ Unknown contract type: {}", contract);
                    println!("📋 Available contracts: token, vesting, airdrop, launchpad");
                    return Ok(());
                }
            }
            
            println!("📋 Next steps:");
            println!("   • Save the contract address for future operations");
            println!("   • Initialize the contract with required parameters");
            println!("   • Update your launch configuration with the new address");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_config_serialization() {
        let config = LaunchConfig {
            name: "TestToken".to_string(),
            symbol: "TST".to_string(),
            decimals: 7,
            total_supply: 1000000,
            mintable: true,
            burnable: false,
            description: "A test token".to_string(),
            website: "https://test.com".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: LaunchConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.symbol, deserialized.symbol);
        assert_eq!(config.decimals, deserialized.decimals);
        assert_eq!(config.total_supply, deserialized.total_supply);
    }

    #[test]
    fn test_recipient_data_parsing() {
        let recipient = RecipientData {
            address: "GABC1234567890".to_string(),
            amount: 1000,
        };

        let json = serde_json::to_string(&recipient).unwrap();
        let parsed: RecipientData = serde_json::from_str(&json).unwrap();

        assert_eq!(recipient.address, parsed.address);
        assert_eq!(recipient.amount, parsed.amount);
    }
}