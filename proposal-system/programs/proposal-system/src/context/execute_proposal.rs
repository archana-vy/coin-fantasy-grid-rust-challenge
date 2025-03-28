use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::storage_accounts::{Multisig, Proposal};

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(mut, has_one = multisig, close = executor)]
    pub proposal: Account<'info, Proposal>,

    #[account(signer)]
    pub executor: Signer<'info>,

    #[account(mut)]
    pub from: Option<UncheckedAccount<'info>>, // For SOL transfer

    #[account(mut)]
    pub to: Option<UncheckedAccount<'info>>, // For SOL transfer

    #[account(mut)]
    pub from_ata: Option<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub to_ata: Option<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// CHECK: This account is unchecked because it is required as the mint authority for token minting.
    #[account(mut)]
    pub mint_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>, // Ensures the correct SPL token program is used
}
