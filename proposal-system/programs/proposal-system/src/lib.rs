pub(crate) mod storage_accounts;
pub(crate) mod context;

use context::*;

use anchor_lang::prelude::*;

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

    pub fn create_multisig(_ctx: Context<CreateMultisig>) -> Result<()> {
        Ok(())
    }

    pub fn add_signer(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn remove_signer(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_proposal(_ctx: Context<CreateProposal>) -> Result<()> {
        Ok(())
    }

    pub fn approve_proposal(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn revoke_approval(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn execute_proposal(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
