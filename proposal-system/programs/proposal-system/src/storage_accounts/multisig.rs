use anchor_lang::prelude::*;

pub(crate) const MAX_NAME_LENGTH: usize = 32;
pub(crate) const MAX_SIGNATORIES: usize = 100;

#[account]
#[derive(Debug)]
pub struct Multisig {
    pub name: String,
    pub creator: Pubkey,
    pub signers: Vec<Pubkey>,
    pub threshold: u32,
    pub proposal_count: u64,
}

impl Multisig {
    pub const MAX_SIZE: usize = (4 + MAX_NAME_LENGTH) + 32 + (4 + 32 * MAX_SIGNATORIES) + 4 + 8;  // 4 bytes (String len) + MAX_NAME_LENGTH + Pubkey size + Vec len + (Pubkey size * MAX_SIGNATORIES) + u32 size + u64 size
}
