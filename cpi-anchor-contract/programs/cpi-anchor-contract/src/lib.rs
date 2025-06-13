use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("CLUys3xRxGXLAvAV6yHxXdNBkBs3a33ZGqqD5NU7VCUD");

#[program]
pub mod cpi_anchor_contract {
    use super::*;

    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, amount)?;

        Ok(())
    }

    //? More generic way for cpi
    pub fn sol_transfer_generic(ctx: Context<SolTransferGeneric>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender_2.to_account_info();
        let to_pubkey = ctx.accounts.recipient_2.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        //? Prepare instruction AccountsMetas
        let account_metas = vec![
            AccountMeta::new(from_pubkey.key(), true),
            AccountMeta::new(to_pubkey.key(), false),
        ];

        //? SOL transfer instruction discriminator
        let instruction_discriminator: u32 = 2; // 2 is for transfersol instruction in system program

        //? Prepare instruction data
        let mut instruction_data = Vec::with_capacity(4 + 8); // first 4 bytes for instruction_discriminator and next 8 bytes for actual lamports which we passing
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes()); // to_le_bytes() converts the number into a little-endian byte array ( least significant byte comes first).
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        //? Create instruction
        let instruction = Instruction {
            program_id: program_id.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        //? Invoke instruction
        invoke(&instruction, &[from_pubkey, to_pubkey, program_id])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SolTransferGeneric<'info> {
    #[account(mut)]
    sender_2: Signer<'info>,
    #[account(mut)]
    recipient_2: SystemAccount<'info>, // checking the owner of this account is system program
    system_program: Program<'info, System>,
}
