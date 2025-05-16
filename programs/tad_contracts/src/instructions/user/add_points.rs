use crate::state::{Config, User};
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow occurred while updating points.")]
    Overflow,
}

pub fn add_points_for_service(ctx: Context<AddPointsForService>, points_to_add: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.points = user
        .points
        .checked_add(points_to_add)
        .ok_or_else(|| error!(ErrorCode::Overflow))?;

    Ok(())
}

#[derive(Accounts)]
pub struct AddPointsForService<'info> {
    #[account(seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub user: Account<'info, User>,

    #[account(address = config.admin)]
    pub admin: Signer<'info>,
}
