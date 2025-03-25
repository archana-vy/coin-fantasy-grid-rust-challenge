use anchor_lang::prelude::*;

use super::MAX_SIGNATORIES;

const MAX_CALLDATA_SIZE: usize = 500;

#[account]
#[derive(Debug)]
pub struct Proposal {
    pub multisig: Pubkey,
    pub creator: Pubkey,
    pub calldata: Vec<u8>,
    pub approvals: Vec<Pubkey>,
    pub executed: bool,
}

impl Proposal {
    pub const MAX_SIZE: usize =
        32 + 32 + (4 + MAX_CALLDATA_SIZE) + (4 + (32 * MAX_SIGNATORIES)) + 1; // Pubkey size + Pubkey size + Vec len + (Pubkey size * MAX_SIGNATORIES) + Vec len + (u8 size * MAX_CALLDATA_SIZE) + bool size
}
