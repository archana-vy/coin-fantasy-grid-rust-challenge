use anchor_lang::prelude::*;

const MAX_MULTISIG_ACCOUNTS: usize = 1000;

#[account]
#[derive(Debug)]
pub struct MultisigProgram {
    // initializer just to track multisig accounts created using the program
    pub multisig_accounts: Vec<Pubkey>,
    // add more parameters if requires
}

impl MultisigProgram {
    pub const MAX_SIZE: usize = 4 + (32 * MAX_MULTISIG_ACCOUNTS); // 4 bytes (Vec len) + (Pubkey size * MAX_MULTISIG_ACCOUNTS)
}
