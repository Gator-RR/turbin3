use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid Program ID")]
    InvalidProgramID,
    #[msg("Invalid Account Counnts")]
    InvalidAccountCount,
    #[msg("Signature is not verifiable")]
    UnverifiableSignature,
    #[msg("CID is invalid")]
    InvalidCID,
}