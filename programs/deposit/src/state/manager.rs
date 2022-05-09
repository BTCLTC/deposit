use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Default)]
pub struct Manager {
    /// owner
    pub owner: Pubkey,
}
