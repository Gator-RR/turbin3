pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("8FHD6Wi2VkqFJ1GoduSiLvKVqMcR7kunJ7Hd4oBSHvpV");

#[program]
pub mod anchor_bunked {
    use super::*;

    pub fn register(ctx: Context<Register>, phash: u64, cid: String, image_hash: [u8; 32], sig: [u8; 64], mint:Option<Pubkey>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.register(phash, cid, image_hash, sig, mint, &ctx.bumps)?;
        Ok(())
    }
}
