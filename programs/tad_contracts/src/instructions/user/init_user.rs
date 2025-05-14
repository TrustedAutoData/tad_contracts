use crate::state::user::User;
use anchor_lang::prelude::*;

pub fn init_user(ctx: Context<InitUser>, email: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.authority = ctx.accounts.authority.key();
    user.email = email;
    Ok(())
}

#[derive(Accounts)]
#[instruction(email: String)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        space = User::LEN,
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
