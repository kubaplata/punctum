use anchor_lang::prelude::*;
use crate::states::{Punctum, UserAccount};

pub fn initialize_user_account(
    ctx: Context<InitializeUserAccount>
) -> Result<()> {
    let user = &mut ctx.accounts.punctum_user_account;
    let punctum = &mut ctx.accounts.punctum;

    user.points = punctum.initial_points;
    user.spent = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUserAccount<'info> {
    #[account(
        mut,
        seeds = [
            "punctum".as_bytes()
        ],
        bump
    )]
    pub punctum: Account<'info, Punctum>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 128,
        seeds = [
            "punctum_user_account".as_bytes(),
            &user.key().as_ref()
        ],
        bump
    )]
    pub punctum_user_account: Account<'info, UserAccount>,

    #[account()]
    pub system_program: Program<'info, System>
}