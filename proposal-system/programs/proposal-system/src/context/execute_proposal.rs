use anchor_lang::prelude::*;

use crate::storage_accounts::{Multisig, Proposal};

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut, has_one = multisig)]
    pub proposal: Account<'info, Proposal>,
    #[account(signer)]
    pub executor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
