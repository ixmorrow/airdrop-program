use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult,
    pubkey::Pubkey, msg, program::{invoke_signed}, account_info::next_account_info, native_token::LAMPORTS_PER_SOL};
use crate::error::AirdropError::{PDAError, IncorrectAccountError};
use crate::instruction::AirdropEnum;
use spl_token::ID as TOKEN_PROGRAM_ID;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        msg!("unpacking instruction data");
        let instruction = AirdropEnum::unpack(instruction_data)?;
        let account_info_iter = &mut accounts.iter();

        match instruction {
            AirdropEnum::AirdropInstruction { amount } => {
                msg!("Airdrop instruction");
                let _payer = next_account_info(account_info_iter)?;
                let user = next_account_info(account_info_iter)?;
                let mint = next_account_info(account_info_iter)?;
                let mint_auth = next_account_info(account_info_iter)?;
                let token_program = next_account_info(account_info_iter)?;

                msg!("deriving pda mint");
                let (pda, bump) = Pubkey::find_program_address(&[], program_id);

                if *mint_auth.key != pda {
                    msg!("Mint and PDA do not match");
                    return Err(PDAError.into())
                }

                if *token_program.key != TOKEN_PROGRAM_ID {
                    msg!("Incorrect token program");
                    return Err(IncorrectAccountError.into())
                }

                invoke_signed(
                    &spl_token::instruction::mint_to(
                        token_program.key,
                        mint.key,
                        user.key,
                        mint_auth.key,
                        &[],
                        amount*LAMPORTS_PER_SOL,
                    )?,
                    &[mint.clone(), user.clone(), mint_auth.clone()],
                    &[&[&[bump]]],
                )?;
            }
        }
        Ok(())
    }
}
