use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub points: i64,
    pub spent: i64,
}