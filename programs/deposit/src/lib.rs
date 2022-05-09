use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

// 合约地址
declare_id!("GameVSYuAQ1TqHbSXbMVPgCrijbTz2UjpqBnnenpSXAY");

#[program]
mod deposit {
    use super::*;

    /// Initialize a manager with a given owner.
    pub fn init_manager(ctx: Context<InitializeManager>, owner: Pubkey) -> ProgramResult {
        instructions::init_manager::handler(ctx, owner)
    }

    /// Deposit tokens
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        instructions::send::handler(ctx, amount)
    }
}
