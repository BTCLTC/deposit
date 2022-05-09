use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

use crate::state::Manager;
use crate::utils::manager_account;

#[derive(Accounts)]
pub struct InitializeManager<'info> {
    #[account(init, address = manager_account::ID, payer = payer)]
    pub manager: Loader<'info, Manager>,

    pub payer: Signer<'info>,

    /// solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

/// Initialize a new manager with a given owner.
pub fn handler(ctx: Context<InitializeManager>, owner: Pubkey) -> ProgramResult {
    let mut manager = ctx.accounts.manager.load_init()?;

    manager.owner = owner;

    Ok(())
}
