use crate::state::car::Car;
use anchor_lang::prelude::*;

pub fn init_car(ctx: Context<InitCar>, vin: String) -> Result<()> {
    let car = &mut ctx.accounts.car;
    car.vin = vin;
    car.owner = ctx.accounts.owner.key();
    car.dealer = ctx.accounts.dealer.key();
    car.total_km = 0;
    car.service_count = 0;
    car.last_service_timestamp = 0;
    car.obd_bumps = ctx.bumps.car;
    Ok(())
}

#[derive(Accounts)]
#[instruction(vin: String)]
pub struct InitCar<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [b"car", vin.as_bytes()],
        bump,
        space = Car::LEN,
    )]
    pub car: Account<'info, Car>,

    /// CHECK: Dealer public key (not validated here)
    pub dealer: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
