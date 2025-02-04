use anchor_lang::prelude::*;

mod context;
mod state;
mod errors;
use context::*;
use errors::*;
use state::*;

declare_id!("7ZjJpeM7TPowKV1P7AeFuMr6RM82CZyazhLKaiPH5jgc");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn listing(ctx: Context<List>, amount: u64) -> Result<()> {
        ctx.accounts.create_listing(amount, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist();
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_vault()?;
        Ok(())
    }
}