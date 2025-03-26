use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::storage_accounts::{Multisig, Proposal};

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(mut, has_one = multisig)]
    pub proposal: Account<'info, Proposal>,

    #[account(signer)]
    pub executor: Signer<'info>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub mint_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    
    pub token_program: Program<'info, Token>,  // Ensures the correct SPL token program is used
}
