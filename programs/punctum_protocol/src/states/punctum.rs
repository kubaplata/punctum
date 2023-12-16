use anchor_lang::prelude::*;

#[account]
pub struct Punctum {
    pub total_users: i64,
    pub admin: Pubkey,
    pub spendable: bool, // Are points spendable.
    pub initial_points: i64,
}