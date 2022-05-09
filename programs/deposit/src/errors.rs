use anchor_lang::{error, prelude::*};

#[error]
pub enum ErrorCode {
    #[msg("Manager already exists")]
    ManagerAlreadyExists,

    #[msg("math overflowed")]
    MathOverflow,

    #[msg("Attempting to divide by zero")]
    DivideByZero,

    #[msg("Depositor Associated Token Account Invalid")]
    DATAInvalid,

    #[msg("Withdrawer Associated Token Account Invalid")]
    WATAInvalid,

    #[msg("Manager Associated Token Account Invalid")]
    MATAInvalid,
}
