use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, msg
};
use borsh::BorshDeserialize;
use crate::instruction::Instruction;
use crate::error::StakingError;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::Initialize {
            rewards_per_token, } => {
                msg!("Initialize pool");
                process_initialize_pool(program_id, accounts, rewards_per_token)
            }
        _ => {
            Err(StakingError::Invalidinstruction.into())
        }
    }
}