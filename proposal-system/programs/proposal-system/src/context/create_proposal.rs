use anchor_lang::prelude::*;
use anchor_spl::token::{spl_token, Mint};

use crate::storage_accounts::{Multisig, Proposal};

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init,
        payer = creator,
        space = 8 + Proposal::MAX_SIZE // 8 bytes (discriminator) + Proposal::MAX_SIZE
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}
