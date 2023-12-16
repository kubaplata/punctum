use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::PunctumError;

pub fn spend_points(
    ctx: Context<SpendPoints>,
    points: i64,
) -> Result<()> {
    let user_account = &mut ctx.accounts.punctum_user_account;
    
    require!(
        user_account.points >= points, 
        PunctumError::InsufficientPoints
    );

    user_account.points -= points;
    user_account.spent += points;

    Ok(())
}

#[derive(Accounts)]
pub struct SpendPoints<'info> {
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
        mut,
        seeds = [
            "punctum_user_account".as_bytes(),
            &user.key().as_ref()
        ],
        bump
    )]
    pub punctum_user_account: Account<'info, UserAccount>,
}