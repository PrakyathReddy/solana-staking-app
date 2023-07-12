#![fg(feature = "test-bpf")]
use borsh::{BorshDeserialize, BorshSerialize};
use std::{assert_eq, println, vec::Vec};

use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{
    instruction::AccountMeta, signature::Keypair, signature::Signer, system_transaction, transaction::Transaction,
};
use solana_validator::test_validator::TestValidatorGenesis;
use stakingapp::{instruction::Instruction as StakingInstruction, state::PoolStorageAccount};

#[test]
fn initialize_pool() {
    solana_logger::setup_with_default("solana_program_runtime=debug");
    // generate program_id for the staking app
    let program_id: Pubkey = Pubkey::new_unique();
    println!("program_id: {:#?}", program_id);

    // start testing environment
    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("stakingapp", program_id)
        .start();
    let rpc_client = test_validator.get_rpc_client();

    // generate pool authority and storage accounts
    let pool_authority = Keypair::new();
    println!("pool authority: {:#?}", pool_authority.pubkey());
    let pool_storage_account = Keypair::new();
    println!("pool_storage_account: {:#?}", pool_storage_account.pubkey());
}
