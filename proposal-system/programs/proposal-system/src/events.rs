use anchor_lang::prelude::*;

#[event]
pub struct MultisigCreated {
    pub multisig: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalCreated {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalVoted {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalExecuted {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub executor: Pubkey,
    pub timestamp: i64,
}