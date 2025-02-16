use anchor_lang::{prelude::*, solana_program::{self, hash, sysvar::instructions::load_instruction_at_checked}, system_program::{transfer, Transfer}};

use crate::{instruction::ResolveBet, state::Bet};

#[derive(Accounts)]
#[instruction(seed: u128) ]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub house: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = player,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Bet::INIT_SPACE +8,
        bump,
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        address = solana_program::sysvar::instructions::ID,
    )]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}

impl<'info> ResolveBet <'info> {
    pub fn verify_ed25519_signature(&mut self, sig:&[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(index, instruction_sysvar_account_info)?;
        require_keys_eq!(ix.program_id, ed25519_program::ID);
        require_eq!(ix.accounts.len(), 0);
        let signatures = Ed25519InstructionSignature::unpack(sig)?.0;

        require_eq!(signatures.len(),1);
        let signature = &signatures[0];
        require!(signature.is_verifiable, true);
        require_keys_eq!(signature.public_key.unwrap(), self.house.key());
        require_eq!(signature.signature.unwrap().eq(&sig), true);

        require_eq!(signature.message.as_ref().unwrap().eq(&self.bet.to_slice()), true);

        Ok(())
    }

    pub fn resolve_bet (&mut self, sig: &[u8], bumps: &ResulveBetBumps) -> Result<()> {
        let hash = hash(sig).to_bytes();
        
        Ok(())
    }
}