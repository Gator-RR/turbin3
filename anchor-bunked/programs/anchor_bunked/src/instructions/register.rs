use crate::{
    state::{index_account::IndexAccount, manifest_account::ManifestAccount},
    CustomError,
};
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{prelude::*, solana_program::sysvar::clock};
// use solana_program::{hash::hash, sysvar::instructions::load_instruction_at_checked};

use anchor_lang::solana_program::ed25519_program;

#[derive(Accounts)]
#[instruction(phash: u64, cid: String)]
pub struct Register<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init_if_needed,
        payer = creator,
        space = IndexAccount::INIT_SPACE + 8,
        seeds = [b"index", phash.to_le_bytes().as_ref()],
        bump,
    )]
    pub index_account: Account<'info, IndexAccount>,

    #[account(
        init,
        payer = creator,
        space = ManifestAccount::INIT_SPACE + 8,
        seeds = [b"manifest", phash.to_le_bytes().as_ref(), index_account.index.to_le_bytes().as_ref()],
        bump,
    )]
    pub manifest_account: Account<'info, ManifestAccount>,
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is the instructions sysvar account
    pub instructions_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Register<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // let ix = load_instruction_at_checked(0, &self.instructions_sysvar.to_account_info())?;

        // // Check instructions are from currernt program - would this work on devnet? Might just skip for now
        // // require_keys_eq!(ix.program_id, ed25519_program::ID, CustomError::InvalidProgramID);

        // // require_eq!(ix.accounts.len(), 0, CustomError::InvalidAccountCount);

        // let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        // require_eq!(signatures.len(), 1, CustomError::CustomError);
        // let signature = &signatures[0];

        // require!(signature.is_verifiable, CustomError::CustomError);

        // // TODO: need to add another check here maybe? There is only one key, right?
        // // require_keys_eq!(signature.public_key.unwrap(), self.house.key(), CustomError::CustomError);

        // require!(
        //     signature.signature.unwrap().eq(sig),
        //     CustomError::CustomError
        // );

        // require!(
        //     signature
        //         .message
        //         .as_ref()
        //         .unwrap()
        //         .eq(&self.manifest_account.hash),
        //     CustomError::CustomError
        // );

        // https://solana.stackexchange.com/questions/16487/about-verify-signature-with-ed25519-issue?noredirect=1&lq=1
        //  Somthing is wrong with this, will assume sig is ok for now, need ot verify this later

        // This could be used but is a lot more CU expensive, might not care for now: https://crates.io/crates/solana_ed25519_verify

        // let message = self.manifest_account.hash.to_vec();
        // let pubkey = self.manifest_account.creator.key();
        // let pubkey_bytes = pubkey.to_bytes();

        // let signature_verification = ed25519_program::verify(
        //     &pubkey_bytes,
        //     &message,
        //     &sig,
        // );

        // require!(signature_verification.is_ok(), CustomError::UnverifiableSignature);

        Ok(())
    }
    pub fn register(&mut self, phash: u64, cid: String, image_hash: [u8; 32], sig: [u8; 64], mint: Option<Pubkey>, bumps: &RegisterBumps) -> Result<()> {
        require!(
            cid.len() == 46,
            CustomError::InvalidCID
        );

        let clock = Clock::get().unwrap();
        // let index_account = &self.index_account;
        let current_index = if self.index_account.is_initialized {
            let current = self.index_account.index;
            self.index_account.index += 1;
            current
        } else {
            self.index_account.set_inner(IndexAccount{
                phash,
                index: 1,
                is_initialized: true,
                bump: bumps.index_account,
            });
            0
        };

        self.manifest_account.set_inner(ManifestAccount{
            creator: self.creator.key(),
            timestamp: clock.unix_timestamp,
            phash,
            index: current_index,
            cid,
            signature: sig,
            hash: image_hash,
            bump: bumps.manifest_account,
            mint,
        });

        msg!(
            "Created manifest at index {} with phash {}",
            current_index,
            phash
        );

        Ok(())
    }
}


