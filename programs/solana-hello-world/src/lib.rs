use anchor_lang::prelude::*;

declare_id!("AYy6VYeespnPnVb6c7sbkdX1irZ5vQxgRtpbCUjJx9mr");

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
