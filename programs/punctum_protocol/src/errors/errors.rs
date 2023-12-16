use anchor_lang::prelude::*;

#[error_code]
pub enum PunctumError {
    #[msg("User does not have sufficient points to perform spending.")]
    InsufficientPoints,
}