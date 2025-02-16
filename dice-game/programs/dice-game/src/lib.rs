pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9YVFhdfq5D6y7HTUMwqMkHnZUUE6PyaS5Mr9ZBzwcJzu");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)?;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, roll: u8, amount: u64) -> Result<()> {
        ctx.accounts.create_bet(seed, roll, amount, &ctx.bumps)?;
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn resolve_bet(ctx: Contect<ResolveBet>, sig: Vec<u8> ) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, &ctx.bumps)?;

        Ok(())
    }
}
