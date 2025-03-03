use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ManifestAccount {
    pub creator: Pubkey,
    pub timestamp: i64,
    pub phash: u64, 
    pub index: u64, // assuming never will be more collisions than 18_446_744_073_709_551_615u64
    #[max_len(46)]
    pub cid: String,  // Might be able to change to a array of ints?
    pub signature: [u8;64], // Ed25519 signature
    pub hash: [u8; 32], // SHA-256
    pub bump: u8,
    pub mint: Option<Pubkey>,
}