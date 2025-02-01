use anchor_lang:: prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::state::Escrow;


#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker,
        constraint = maker.key() == escrow.maker.key()
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        token::authority = escrow,
        token::mint = mint_a,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info,AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl <'info> Refund <'info> {
    pub fn refund(&mut self)-> Result<()> {
        let cpi_program= self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            to: self.maker_ata_a.to_account_info(),
            from: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.escrow.maker.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],            
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;
        Ok(())
    }

    pub fn close_refund(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.escrow.maker.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let cpi_ctx =CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        close_account(cpi_ctx)?;
        Ok(())
    }
}