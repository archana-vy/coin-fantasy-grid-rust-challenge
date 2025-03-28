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

    use anchor_spl::associated_token::get_associated_token_address;

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
            !proposal.votes.contains(&voter),
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
                let to_ata = get_associated_token_address(&to, &ctx.accounts.mint.key());

                let to_account = ctx
                    .accounts
                    .to
                    .as_ref()
                    .ok_or(MultisigErrors::MintToAccountIsNull)?;

                require!(to_account.key() == to, MultisigErrors::InvalidMintToPubkey);

                let to_ata_account = ctx
                    .accounts
                    .to_ata
                    .as_ref()
                    .ok_or(MultisigErrors::MintToTokenAccountIsNull)?;

                require!(
                    to_ata_account.key() == to_ata,
                    MultisigErrors::InvalidMintToATA
                );

                let cpi_accounts = MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: to_ata_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                };
                let cpi_ctx =
                    CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
                token::mint_to(cpi_ctx, amount)?;
            }
            InstructionType::Transfer { from, to, amount } => {
                let from_account = ctx
                    .accounts
                    .from
                    .as_ref()
                    .ok_or(MultisigErrors::TransferFromAccountIsNull)?;

                require!(
                    from_account.key() == from,
                    MultisigErrors::InvalidTransferFromPubkey
                );

                let from_ata_account = ctx
                    .accounts
                    .from_ata
                    .as_ref()
                    .ok_or(MultisigErrors::TransferFromTokenAccountIsNull)?;

                let from_ata = get_associated_token_address(&from, &ctx.accounts.mint.key());

                require!(
                    from_ata_account.key() == from_ata,
                    MultisigErrors::InvalidTransferFromATA
                );

                let to_account = ctx
                    .accounts
                    .to
                    .as_ref()
                    .ok_or(MultisigErrors::TransferToAccountIsNull)?;

                require!(
                    to_account.key() == to,
                    MultisigErrors::InvalidTransferToPubkey
                );

                let to_ata_account = ctx
                    .accounts
                    .to_ata
                    .as_ref()
                    .ok_or(MultisigErrors::TransferToTokenAccountIsNull)?;

                let to_ata = get_associated_token_address(&to, &ctx.accounts.mint.key());

                require!(
                    to_ata_account.key() == to_ata,
                    MultisigErrors::InvalidTransferToATA
                );

                let cpi_accounts = Transfer {
                    from: from_ata_account.to_account_info(),
                    to: to_ata_account.to_account_info(),
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
                let seller_account = ctx
                    .accounts
                    .from
                    .as_ref()
                    .ok_or(MultisigErrors::SellerAccountIsNull)?;

                require!(
                    seller_account.key() == seller,
                    MultisigErrors::InvalidSellerPubkey
                );

                let seller_ata_account = ctx
                    .accounts
                    .from_ata
                    .as_ref()
                    .ok_or(MultisigErrors::SellerTokenAccountIsNull)?;

                let seller_ata = get_associated_token_address(&seller, &ctx.accounts.mint.key());

                require!(
                    seller_ata_account.key() == seller_ata,
                    MultisigErrors::InvalidSellerATA
                );

                let buyer_account = ctx
                    .accounts
                    .to
                    .as_ref()
                    .ok_or(MultisigErrors::BuyerAccountIsNull)?;

                require!(
                    buyer_account.key() == buyer,
                    MultisigErrors::InvalidBuyerPubkey
                );

                let buyer_ata_account = ctx
                    .accounts
                    .to_ata
                    .as_ref()
                    .ok_or(MultisigErrors::BuyerTokenAccountIsNull)?;

                let buyer_ata = get_associated_token_address(&buyer, &ctx.accounts.mint.key());

                require!(
                    buyer_ata_account.key() == buyer_ata,
                    MultisigErrors::InvalidBuyerATA
                );

                // Transfer SOL from buyer to seller
                invoke(
                    &system_instruction::transfer(&seller, &buyer, sol_price),
                    &[
                        buyer_account.to_account_info(),
                        seller_account.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;

                let cpi_accounts = Transfer {
                    from: seller_ata_account.to_account_info(),
                    to: buyer_ata_account.to_account_info(),
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
                let seller_account = ctx
                    .accounts
                    .from
                    .as_ref()
                    .ok_or(MultisigErrors::SellerAccountIsNull)?;

                require!(
                    seller_account.key() == seller,
                    MultisigErrors::InvalidSellerPubkey
                );

                let seller_ata_account = ctx
                    .accounts
                    .from_ata
                    .as_ref()
                    .ok_or(MultisigErrors::SellerTokenAccountIsNull)?;

                let seller_ata = get_associated_token_address(&seller, &ctx.accounts.mint.key());

                require!(
                    seller_ata_account.key() == seller_ata,
                    MultisigErrors::InvalidSellerATA
                );

                let buyer_account = ctx
                    .accounts
                    .to
                    .as_ref()
                    .ok_or(MultisigErrors::BuyerAccountIsNull)?;

                require!(
                    buyer_account.key() == buyer,
                    MultisigErrors::InvalidBuyerPubkey
                );

                let buyer_ata_account = ctx
                    .accounts
                    .to_ata
                    .as_ref()
                    .ok_or(MultisigErrors::BuyerTokenAccountIsNull)?;

                let buyer_ata = get_associated_token_address(&buyer, &ctx.accounts.mint.key());

                require!(
                    buyer_ata_account.key() == buyer_ata,
                    MultisigErrors::InvalidBuyerATA
                );

                // Transfer SOL from buyer to seller
                invoke(
                    &system_instruction::transfer(&seller, &buyer, sol_price),
                    &[
                        buyer_account.to_account_info(),
                        seller_account.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;

                let cpi_accounts = Transfer {
                    from: seller_ata_account.to_account_info(),
                    to: buyer_ata_account.to_account_info(),
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
