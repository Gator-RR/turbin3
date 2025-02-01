use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;


declare_id!("CmwenF4PBUNjL9Efxvm9sixvf2eZnSbFi4MfCn5RTK9a");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.transfer_to_maker()?;


        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_refund()?;
        Ok(())
    }
}