use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Define the type of state stored in
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolStorageAccount {
    pub pool_authority: Pubkey,
    pub total_staked: u64,
    pub user_count: u64,
    pub rewards_per_token: u64,
}