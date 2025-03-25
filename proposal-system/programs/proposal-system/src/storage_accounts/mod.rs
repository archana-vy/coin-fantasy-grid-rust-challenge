mod multisig;
mod multisig_program;
mod proposal;

pub(crate) use multisig_program::MultisigProgram;
pub(crate) use multisig::{Multisig, MAX_SIGNATORIES};
pub(crate) use proposal::Proposal;