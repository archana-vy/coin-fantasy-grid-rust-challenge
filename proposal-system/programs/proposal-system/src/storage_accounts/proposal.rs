use anchor_lang::prelude::*;

use super::MAX_SIGNATORIES;

pub(crate) const MAX_CALLDATA_SIZE: usize = 32 + 32 + 8 + 8;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum InstructionType {
    Mint {
        to: Pubkey, // to user's associated token account
        amount: u64,
    },
    Transfer {
        from: Pubkey, // from user's associated token account
        to: Pubkey,   // to user's associated token account
        amount: u64,
    },
    Buy {
        seller: Pubkey, // seller's associated token account
        buyer: Pubkey,  // buyer's associated token account
        token_amount: u64,
        sol_price: u64,
    },
    Sell {
        seller: Pubkey, // seller's associated token account
        buyer: Pubkey,  // buyer's associated token account
        token_amount: u64,
        sol_price: u64,
    },
}

#[account]
#[derive(Debug)]
pub struct Proposal {
    pub multisig: Pubkey,
    pub creator: Pubkey,
    pub instruction: InstructionType,
    pub mint: Pubkey, // Token mint address
    pub votes: Vec<Pubkey>,
    pub executed: bool,
}

impl Proposal {
    pub const MAX_SIZE: usize =
        32 + 32 + (4 + MAX_CALLDATA_SIZE) + (4 + (32 * MAX_SIGNATORIES)) + 1; // Pubkey size + Pubkey size + Vec len + (Pubkey size * MAX_SIGNATORIES) + Vec len + (u8 size * MAX_CALLDATA_SIZE) + bool size
}
