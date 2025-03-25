use anchor_lang::prelude::*;

use crate::storage_accounts::Proposal;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Proposal::MAX_SIZE // 8 bytes (discriminator) + Proposal::MAX_SIZE
    )]
    pub Proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}