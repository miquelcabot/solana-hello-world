use anchor_lang::prelude::*;

declare_id!("3XoiyqxN7a4exZ6MHUcaHn7xjvpmBs82CmGAJRohM1DP");

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
