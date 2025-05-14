use crate::state::user::User;
use anchor_lang::prelude::*;

pub fn init_user(ctx: Context<InitUser>, email: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.wallet = ctx.accounts.wallet.key();
    user.email = email;
    Ok(())
}

#[derive(Accounts)]
#[instruction(email: String)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = wallet,
        seeds = [b"user", wallet.key().as_ref()],
        bump,
        space = User::LEN,
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}
