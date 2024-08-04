use anchor_lang::prelude::*;

declare_id!("J4cWWm4X94upYcFKLtr7WSeZchrXRZ59szMBcDpAe8bt");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        msg!("Account address: {}",my_account.key());
        my_account.data = 0;
        my_account.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn increase(ctx: Context<Update>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data += 1;
        Ok(())
    }

    pub fn decrease(ctx: Context<Update>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        if my_account.data == 0 {
            return Err(error!(ErrorCode::BelowZero));
        }
        my_account.data -= 1;
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub my_account: Account<'info, MyAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    authority: Pubkey,
    data: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrease value 0f 0.")]
    BelowZero,
}
