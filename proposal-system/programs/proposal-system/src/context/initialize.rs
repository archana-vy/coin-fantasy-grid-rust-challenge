use anchor_lang::prelude::*;

use crate::storage_accounts::MultisigProgram;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + MultisigProgram::MAX_SIZE // 8 bytes (discriminator) + MultisigProgram::MAX_SIZE
    )]
    pub multisig_program: Account<'info, MultisigProgram>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
