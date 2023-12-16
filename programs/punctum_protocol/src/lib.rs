use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod states;
pub use states::*;

pub mod errors;
pub use errors::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod punctum_protocol {
    use super::*;

    pub fn initialize_main(
        ctx: Context<InitializeMain>,
        spendable: bool,
        initial_points: i64,
    ) -> Result<()> {
        instructions::initialize_main(
            ctx,
            spendable,
            initial_points
        )
    }

    pub fn initialize_user_account(
        ctx: Context<InitializeUserAccount>,
    ) -> Result<()> {
        instructions::initialize_user_account(
            ctx
        )
    }

    pub fn add_points(
        ctx: Context<AddPoints>,
        points: i64,
    ) -> Result<()> {
        instructions::add_points(
            ctx, 
            points
        )
    }

    pub fn spend_points(
        ctx: Context<SpendPoints>,
        points: i64,
    ) -> Result<()> {
        instructions::spend_points(
            ctx,
            points
        )
    }
}
