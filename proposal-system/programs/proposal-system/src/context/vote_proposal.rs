use anchor_lang::prelude::*;

use crate::storage_accounts::{Multisig, Proposal};

#[derive(Accounts)]
pub struct VoteProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub signer: Signer<'info>,
}
