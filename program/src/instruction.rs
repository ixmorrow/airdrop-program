use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use crate::error::AirdropError::{InstructionUnpackError};
use solana_program::{program_error::ProgramError};


#[derive(BorshDeserialize, Debug)]

struct AirdropPayload {
    amount: u64,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum AirdropEnum {
    AirdropInstruction {
        amount: u64
    }
}

impl AirdropEnum {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (variant, amount) = input.split_first().ok_or(InstructionUnpackError)?;
        let payload = AirdropPayload::try_from_slice(amount).unwrap();
        Ok(match variant {
            0 => Self::AirdropInstruction { amount: payload.amount },
            _ => return Err(InstructionUnpackError.into())
        })
    }
}