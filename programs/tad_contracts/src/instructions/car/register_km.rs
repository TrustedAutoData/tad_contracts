use crate::{events::KmRegistered, state::car::Car};
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow occurred while updating kilometers.")]
    Overflow,
}

pub fn register_km(ctx: Context<RegisterKm>, new_km: u64) -> Result<()> {
    let car = &mut ctx.accounts.car;
    car.total_km = car
        .total_km
        .checked_add(new_km)
        .ok_or_else(|| error!(ErrorCode::Overflow))?;

    msg!(
        "KM REGISTERED → VIN: {}, Added: {}, Total: {}",
        car.vin,
        new_km,
        car.total_km
    );

    emit!(KmRegistered {
        vin: car.vin.clone(),
        new_km,
        updated_total: car.total_km,
    });

    Ok(())
}

#[derive(Accounts)]

pub struct RegisterKm<'info> {
    #[account(
        mut,
        seeds = [b"car", car.vin.as_bytes()],
        bump = car.obd_bumps,
        has_one = owner,
    )]
    pub car: Account<'info, Car>,

    #[account()]
    pub owner: UncheckedAccount<'info>,
}
