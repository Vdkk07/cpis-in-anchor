use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};
declare_id!("6sSQXx7SWSCd7uzJczyrvB8GEN4aHiaeVfwWpsipJCa5");

#[program]
pub mod cpi_into_native_contract {
    use super::*;

    pub fn init(ctx: Context<Initialize>, value: u32) -> Result<()> {
        msg!("Initialize data account");
        let data_account = ctx.accounts.data_account.to_account_info();
        let user_account = ctx.accounts.user_account.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        let cpi_program = ctx.accounts.cpi_program.to_account_info();

        let account_metas = vec![
            AccountMeta::new(*data_account.key, true),
            AccountMeta::new(*user_account.key, true),
            AccountMeta::new_readonly(*system_program.key, false),
        ];

        let instruction = Instruction {
            program_id: *cpi_program.key,
            accounts: account_metas,
            data: vec![0], // CounterInstruction::Init
        };

        invoke(
            &instruction,
            &[data_account, user_account, system_program, cpi_program],
        )?;

        Ok(())
    }

    pub fn double(ctx: Context<Modify>) -> Result<()> {
        msg!("Double data account value");
        let data_account = ctx.accounts.data_account.to_account_info();
        let cpi_program_id = ctx.accounts.cpi_program.key;

        let accounts = vec![AccountMeta::new(*data_account.key, false)];

        let instruction = Instruction {
            program_id: *cpi_program_id,
            accounts: accounts,
            data: vec![1], // CounterInstruction::Double
        };

        invoke(&instruction, &[data_account])?;

        Ok(())
    }
    pub fn half(ctx: Context<Modify>) -> Result<()> {
        msg!("Half data account value");
        let data_account = ctx.accounts.data_account.to_account_info();
        let cpi_program_id = ctx.accounts.cpi_program.key;

        let accounts = vec![AccountMeta::new(*data_account.key, false)];

        let instruction = Instruction {
            program_id: *cpi_program_id,
            accounts: accounts,
            data: vec![2], // CounterInstruction::Half
        };

        invoke(&instruction, &[data_account])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is a raw account passed to the native program, and the native program handles validation
    #[account(mut)]
    data_account: AccountInfo<'info>,

    /// CHECK: This is a signer used for account creation, validated by anchor
    #[account(mut)]
    user_account: Signer<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is the native program we CPI into, no need to validate
    pub cpi_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    /// CHECK: This is a raw account owned by the native program and is validated in the native logic
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// CHECK: Used only as signer reference for seed validation or PDA derivation
    pub user_account: AccountInfo<'info>,

    /// CHECK: The native program to CPI into
    pub cpi_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     data_account: AccountInfo<'info>,
//     #[account(mut)]
//     user_account: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//     pub cpi_program: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct Modify<'info> {
//     #[account(mut)]
//     pub data_account: AccountInfo<'info>,
//     pub cpi_program: AccountInfo<'info>,
// }
