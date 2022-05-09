use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::associated_token::{self, create, Create};
use anchor_spl::token::{self, Mint, Transfer};
use spl_associated_token_account::get_associated_token_address;

use crate::errors::ErrorCode;
use crate::state::Manager;

#[event]
pub struct DepositEvent {
    depositor: Pubkey,
    mint: Pubkey,
    amount: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub manager: Loader<'info, Manager>,

    #[account(address = manager.load()?.owner)]
    pub owner: AccountInfo<'info>,

    /// owner ata
    #[account(mut)]
    pub owner_account: AccountInfo<'info>,

    /// The user/authority that owns the deposit
    pub depositor: Signer<'info>,

    /// Token mint
    pub token_mint: Account<'info, Mint>,

    /// depositor ata
    #[account(mut)]
    pub deposit_account: AccountInfo<'info>,

    /// current cluster rent
    pub rent: Sysvar<'info, Rent>,

    /// spl token program
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,

    /// associated id
    #[account(address = associated_token::ID)]
    pub associated_program: AccountInfo<'info>,

    /// solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    fn create_account_context(&self) -> CpiContext<'_, '_, '_, 'info, Create<'info>> {
        CpiContext::new(
            self.associated_program.clone(),
            Create {
                payer: self.depositor.to_account_info(),
                associated_token: self.owner_account.to_account_info(),
                authority: self.owner.to_account_info(),
                mint: self.token_mint.to_account_info(),
                system_program: self.system_program.to_account_info(),
                token_program: self.token_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        )
    }

    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.clone(),
            Transfer {
                from: self.deposit_account.to_account_info(),
                to: self.owner_account.to_account_info(),
                authority: self.depositor.to_account_info(),
            },
        )
    }
}

/// Deposit tokens into supply account.
pub fn handler(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
    let mint = ctx.accounts.token_mint.key();

    let deposit_account = get_associated_token_address(&ctx.accounts.depositor.key(), &mint);

    // check depositor ata
    if deposit_account != ctx.accounts.deposit_account.key() {
        return Err(ErrorCode::DATAInvalid.into());
    }

    let owner_account = get_associated_token_address(&ctx.accounts.owner.key(), &mint);

    // check owner ata
    if owner_account != ctx.accounts.owner_account.key() {
        return Err(ErrorCode::MATAInvalid.into());
    }

    if ctx.accounts.owner_account.owner != &token::ID {
        create(ctx.accounts.create_account_context())?;
    }

    token::transfer(ctx.accounts.transfer_context(), amount)?;

    emit!(DepositEvent {
        depositor: ctx.accounts.depositor.key(),
        mint: ctx.accounts.token_mint.key(),
        amount,
    });

    Ok(())
}
