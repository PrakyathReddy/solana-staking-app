use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Copy, Debug, Clone)]
pub enum StakingError {
    /// Invalid instruction
    #[error("Invalid instruction")]
InvalidInstruction, }

impl From for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
// impl From<StakingError> for ProgramError is just a convertor from our error to Solana error.