#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Vec};

#[contract]
pub struct SplitPayment;

#[derive(Clone)]
#[contracttype]
pub struct Split {
    pub recipients: Vec<Address>,
    pub shares: Vec<u64>,
    pub total_received: u64,
    pub distributed: bool,
}

#[contractimpl]
impl SplitPayment {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Admin creates a payment split
    /// - split_id: unique identifier for the split
    /// - recipients: list of recipient addresses
    /// - shares: percentage shares (must sum to 100)
    pub fn create_split(env: Env, admin: Address, split_id: u64, recipients: Vec<Address>, shares: Vec<u64>) {
        admin.require_auth();

        // Validate shares sum to 100
        let mut sum: u64 = 0;
        for i in 0..shares.len() {
            sum += shares.get(i).unwrap();
        }
        if sum != 100 {
            panic!("Shares must sum to 100");
        }

        if recipients.len() != shares.len() {
            panic!("Recipients and shares must have same length");
        }

        let split = Split {
            recipients,
            shares,
            total_received: 0,
            distributed: false,
        };

        let mut splits: Map<u64, Split> = env
            .storage()
            .instance()
            .get(&"splits")
            .unwrap_or(Map::new(&env));

        splits.set(split_id, split);
        env.storage().instance().set(&"splits", &splits);
    }

    /// Anyone can contribute to a split
    /// The sender must authorize the transfer
    pub fn contribute(env: Env, split_id: u64) {
        let mut splits: Map<u64, Split> = env
            .storage()
            .instance()
            .get(&"splits")
            .unwrap_or(Map::new(&env));

        let split = splits.get(split_id).unwrap_or_else(|| panic!("Split not found"));

        if split.distributed {
            panic!("Split already distributed");
        }

        // Update total_received (we need to track this separately as we can't easily get sent amount in contract)
        // Note: In a real implementation, you would track the balance change
        let mut new_split = split;
        new_split.total_received += 1; // Placeholder - actual amount tracking requires token integration

        splits.set(split_id, new_split);
        env.storage().instance().set(&"splits", &splits);
    }

    /// Admin triggers distribution of funds to recipients
    pub fn distribute(env: Env, admin: Address, split_id: u64) {
        admin.require_auth();

        let mut splits: Map<u64, Split> = env
            .storage()
            .instance()
            .get(&"splits")
            .unwrap_or(Map::new(&env));

        let split = splits.get(split_id).unwrap_or_else(|| panic!("Split not found"));

        if split.distributed {
            panic!("Split already distributed");
        }

        // Note: Actual token transfer would require token contract integration
        // This is a placeholder for the distribution logic

        let mut new_split = split;
        new_split.distributed = true;

        splits.set(split_id, new_split);
        env.storage().instance().set(&"splits", &splits);
    }

    /// Get split details
    pub fn get_split(env: Env, split_id: u64) -> (Vec<Address>, Vec<u64>, u64, bool) {
        let splits: Map<u64, Split> = env
            .storage()
            .instance()
            .get(&"splits")
            .unwrap_or(Map::new(&env));

        let split = splits.get(split_id).unwrap_or_else(|| panic!("Split not found"));
        (split.recipients, split.shares, split.total_received, split.distributed)
    }
}
