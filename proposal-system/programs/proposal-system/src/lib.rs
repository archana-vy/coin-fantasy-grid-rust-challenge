use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;

#[cfg(feature = "devnet")]
const PROGRAM_ID: Pubkey = Pubkey::from_str_const("2F3xVyraFXuZpCShqg2YVuF5HxNukmGtGdkm5fJ41L3R");


// Fallback for when no feature is specified
#[cfg(not(any(feature = "devnet")))]
const PROGRAM_ID: Pubkey = Pubkey::from_str_const("D9es5JgK89P3DHSDdD2bFnn3iYzc5ygRA789Sp4VUtJ2"); // default local

declare_id!(PROGRAM_ID);

#[program]
pub mod proposal_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let multisig_program = &mut ctx.accounts.multisig_program;
        multisig_program.multisig_accounts = Vec::new();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4 + (32 * 100) // 8 bytes (discriminator) + 4 bytes (Vec len) + (Pubkey size * max no. of accounts)
    )]
    pub multisig_program: Account<'info, MultisigProgram>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug)]
pub struct MultisigProgram {
    // initializer just to track multisig accounts created using the program
    pub multisig_accounts: Vec<Pubkey>,
    // add more parameters if requires
}

#[account]
pub struct Multisig {
    pub name: String,
    pub creator: Pubkey,
    pub signers: Vec<Pubkey>,
    pub threshold: u32,
    pub proposal_count: u64,
}

#[account]
pub struct Proposal {
    pub multisig: Pubkey,
    pub creator: Pubkey,
    pub calldata: Vec<u8>,
    pub approvals: Vec<Pubkey>,
    pub executed: bool,
}

