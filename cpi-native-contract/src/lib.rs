use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::INVALID_INSTRUCTION_DATA,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
struct CounterState {
    count: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum CounterInstruction {
    Init,
    Double,
    Half,
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| INVALID_INSTRUCTION_DATA)?;

    match instruction {
        CounterInstruction::Init => {
            msg!("Initializing counter");
            let iter = &mut accounts.iter();
            let data_account = next_account_info(iter)?;
            let payer = next_account_info(iter)?;
            let system_program = next_account_info(iter)?;

            // Check if payer is signer
            if !payer.is_signer {
                return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
            }

            // Calculate space needed for CounterState
            let space = 8;

            // Calculate rent exemption amount
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);

            // Create the account
            let create_account_ix = create_account(
                payer.key,
                data_account.key,
                lamports,
                space as u64,
                program_id,
            );

            invoke(
                &create_account_ix,
                &[payer.clone(), data_account.clone(), system_program.clone()],
            )?;

            // Initialize the data account
            let counter = CounterState { count: 1 };
            counter.serialize(&mut *data_account.data.borrow_mut())?;
        }

        CounterInstruction::Double => {
            let iter = &mut accounts.iter();
            let data_account = next_account_info(iter)?;
            // Check if the account is owned by this program
            if data_account.owner != program_id {
                return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
            }

            let mut counter = CounterState::try_from_slice(&data_account.data.borrow())?;
            counter.count = counter.count.saturating_mul(2);
            counter.serialize(&mut *data_account.data.borrow_mut())?;
        }

        CounterInstruction::Half => {
            let iter = &mut accounts.iter();
            let data_account = next_account_info(iter)?;
            // Check if the account is owned by this program
            if data_account.owner != program_id {
                return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
            }

            let mut counter = CounterState::try_from_slice(&data_account.data.borrow())?;
            counter.count = counter.count.saturating_div(2);
            counter.serialize(&mut *data_account.data.borrow_mut())?;
        }
    }
    Ok(())
}
