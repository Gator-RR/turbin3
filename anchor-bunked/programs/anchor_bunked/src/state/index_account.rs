use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IndexAccount{
    pub phash: u64, // Will not work if there are more than 18_446_744_073_709_551_615 - 1 collisions
    pub index: u64,
    pub is_initialized: bool,
    pub bump: u8,
}