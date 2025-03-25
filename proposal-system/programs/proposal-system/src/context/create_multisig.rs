use anchor_lang::prelude::*;

use crate::storage_accounts::Multisig;

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Multisig::MAX_SIZE // 8 bytes (discriminator) + Multisig::MAX_SIZE
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
