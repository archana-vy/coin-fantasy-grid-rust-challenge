pub mod context;
pub mod errors;
pub mod events;
pub mod storage_accounts;

use context::*;
use errors::*;
use events::*;
use storage_accounts::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_spl::token::{self, MintTo, Transfer};

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

    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        name: String,
        signers: Vec<Pubkey>,
        threshold: u32,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(
            name.len() <= MAX_NAME_LENGTH,
            MultisigErrors::MaxAccountNameLength
        );
        require!(
            signers.len() <= MAX_SIGNATORIES,
            MultisigErrors::MultisigCreatorIsNotSigner
        );
        require!(
            (threshold as usize) <= MAX_SIGNATORIES,
            MultisigErrors::MaxThreshold
        );

        multisig.name = name;
        multisig.creator = ctx.accounts.creator.key();
        multisig.signers = signers;
        multisig.threshold = threshold;
        multisig.proposal_count = 0;

        emit!(MultisigCreated {
            multisig: multisig.key(),
            creator: multisig.creator,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // pub fn add_signer(_ctx: Context<AddSigner>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn remove_signer(_ctx: Context<RemoveSigner>) -> Result<()> {
    //     Ok(())
    // }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        instruction: InstructionType,
    ) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let proposal = &mut ctx.accounts.proposal;
        let mint = &mut ctx.accounts.mint;
        let creator = ctx.accounts.creator.key();

        require!(
            multisig.signers.contains(&creator),
            MultisigErrors::InvalidProposer
        );

        proposal.multisig = multisig.key();
        proposal.creator = creator;
        proposal.instruction = instruction;
        proposal.mint = mint.key();
        proposal.votes = Vec::new();
        proposal.executed = false;

        emit!(ProposalCreated {
            multisig: proposal.multisig,
            proposal: proposal.key(),
            creator: proposal.creator,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn vote_proposal(ctx: Context<VoteProposal>) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let proposal = &mut ctx.accounts.proposal;
        let voter = ctx.accounts.signer.key();

        require!(
            multisig.signers.contains(&voter),
            MultisigErrors::InvalidVoter
        );

        require!(
            proposal.votes.contains(&voter),
            MultisigErrors::ProposalAlreadyVoted
        );

        proposal.votes.push(voter);

        emit!(ProposalVoted {
            multisig: proposal.multisig,
            proposal: proposal.key(),
            voter: voter,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let proposal = &mut ctx.accounts.proposal;
        let executor = ctx.accounts.executor.key();

        require!(
            proposal.creator.eq(&executor),
            MultisigErrors::InvalidExecutor
        );

        require!(
            (proposal.votes.len() as u32) >= multisig.threshold,
            MultisigErrors::NotEnoughVotes
        );

        require!(!proposal.executed, MultisigErrors::AlreadyExecuted);

        // CPI logic to execute calldata

        match proposal.instruction {
            InstructionType::Mint { to, amount } => {
                require!(
                    ctx.accounts.to.key() == to,
                    MultisigErrors::InvalidMintToPubkey
                );
                let cpi_accounts = MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                };
                let cpi_ctx =
                    CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
                token::mint_to(cpi_ctx, amount)?;
            }
            InstructionType::Transfer { from, to, amount } => {
                require!(
                    ctx.accounts.from.key() == from,
                    MultisigErrors::InvalidTransferToPubkey
                );
                require!(
                    ctx.accounts.to.key() == to,
                    MultisigErrors::InvalidTransferFromPubkey
                );

                let cpi_accounts = Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                };
                let cpi_ctx =
                    CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
                token::transfer(cpi_ctx, amount)?;
            }
            InstructionType::Buy {
                seller,
                buyer,
                token_amount,
                sol_price,
            } => {
                require!(
                    ctx.accounts.from.key() == seller,
                    MultisigErrors::InvalidTransferSellerPubkey
                );
                require!(
                    ctx.accounts.to.key() == buyer,
                    MultisigErrors::InvalidTransferBuyerPubkey
                );

                // Transfer SOL from buyer to seller
                invoke(
                    &system_instruction::transfer(&seller, &buyer, sol_price),
                    &[
                        ctx.accounts.to.to_account_info(),
                        ctx.accounts.from.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;

                let cpi_accounts = Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                };
                let cpi_ctx =
                    CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
                token::transfer(cpi_ctx, token_amount)?;
            }
            InstructionType::Sell {
                seller,
                buyer,
                token_amount,
                sol_price,
            } => {
                require!(
                    ctx.accounts.from.key() == seller,
                    MultisigErrors::InvalidTransferSellerPubkey
                );
                require!(
                    ctx.accounts.to.key() == buyer,
                    MultisigErrors::InvalidTransferBuyerPubkey
                );

                // Transfer SOL from buyer to seller
                invoke(
                    &system_instruction::transfer(&seller, &buyer, sol_price),
                    &[
                        ctx.accounts.to.to_account_info(),
                        ctx.accounts.from.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;

                let cpi_accounts = Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                };
                let cpi_ctx =
                    CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
                token::transfer(cpi_ctx, token_amount)?;
            }
        };

        proposal.executed = true;

        emit!(ProposalExecuted {
            multisig: proposal.multisig,
            proposal: proposal.key(),
            executor: executor,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}
