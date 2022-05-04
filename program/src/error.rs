use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum AirdropError {
    /// Error description
    #[error("Error unpacking instruction")]
    InstructionUnpackError,
    #[error("Error in PDA validation")]
    PDAError,
    #[error("Incorrect account")]
    IncorrectAccountError,
}

impl PrintProgramError for AirdropError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<AirdropError> for ProgramError {
    fn from(e: AirdropError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for AirdropError {
    fn type_of() -> &'static str {
        "Airdrop error"
    }
}
