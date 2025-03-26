use anchor_lang::error_code;

#[error_code]
pub enum MultisigErrors {
    MaxAccountNameLength,
    MaxSignatories,
    MultisigCreatorIsNotSigner,
    MaxThreshold,
    MaxCalldataSize,
    InvalidProposer,
    InvalidMint,
    InvalidVoter,
    ProposalAlreadyVoted,
    InvalidExecutor,
    NotEnoughVotes,
    AlreadyExecuted,
}
