use anchor_lang::prelude::*;
use crate::states::*;

pub fn add_points(
    ctx: Context<AddPoints>,
    points: i64,
) -> Result<()> {
    let user = &mut ctx.accounts.punctum_user_account;
    user.points += points;

    Ok(())
}

#[derive(Accounts)]
pub struct AddPoints<'info> {
    #[account(
        mut,
        seeds = [
            "punctum".as_bytes()
        ],
        bump
    )]
    pub punctum: Account<'info, Punctum>,

    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            "punctum_user_account".as_bytes(),
            &user.key().as_ref()
        ],
        bump
    )]
    pub punctum_user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        constraint = punctum.admin == admin.key()
    )]
    pub admin: Signer<'info>
}