use anchor_lang::prelude::*;
use crate::states::Punctum;

pub fn initialize_main(
    ctx: Context<InitializeMain>,
    spendable: bool,
    initial_points: i64,
) -> Result<()> {
    let punctum = &mut ctx.accounts.punctum;
    let admin = &mut ctx.accounts.admin;

    punctum.total_users = 0;
    punctum.admin = admin.key();
    punctum.spendable = spendable;
    punctum.initial_points = initial_points;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMain<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 128,
        seeds = [
            "punctum".as_bytes()
        ],
        bump
    )]
    pub punctum: Account<'info, Punctum>,

    #[account()]
    pub system_program: Program<'info, System>
}