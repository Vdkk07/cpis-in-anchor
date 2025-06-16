use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
struct CounterState {
    count: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum CounterInstruction {
    Initialize,
    Double,
    Half,
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let data_account = next_account_info(iter)?;

    if !data_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut counter = CounterState::try_from_slice(&data_account.data.borrow())?;

    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        CounterInstruction::Initialize => {
            msg!("Initialize data account");
            let instruction = Instruction {
                program_id: *program_id,
                accounts: vec![AccountMeta {
                    pubkey: *data_account.key,
                    is_signer: true,
                    is_writable: false,
                }],
                data: vec![],
            };

            invoke(&instruction, &[data_account.clone()])?;
        }

        CounterInstruction::Double => {
            msg!("Double counter");
            counter.count = counter.count.saturating_mul(2);
        }

        CounterInstruction::Half => {
            msg!("Half counter");
            counter.count = counter.count.saturating_div(2);
        }
    }

    counter.serialize(&mut *data_account.data.borrow_mut())?;

    Ok(())
}
