use crate::state::config::Config;
use anchor_lang::prelude::*;

pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
    ctx.accounts.config.admin = ctx.accounts.admin.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(
        init,
        payer = admin,
        seeds = [b"config"],
        bump,
        space = Config::LEN,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
