use crate::state::dealer::Dealer;
use anchor_lang::prelude::*;

pub fn init_dealer(ctx: Context<InitDealer>, name: String) -> Result<()> {
    let dealer = &mut ctx.accounts.dealer;
    dealer.authority = ctx.accounts.authority.key();
    dealer.name = name;
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitDealer<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"dealer", authority.key().as_ref()],
        bump,
        space = Dealer::LEN
    )]
    pub dealer: Account<'info, Dealer>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
