use crate::events::ErrorReported;
use crate::state::car::Car;
use anchor_lang::prelude::*;

pub fn report_error(ctx: Context<ReportError>, error_code: u16, message: String) -> Result<()> {
    let car = &ctx.accounts.car;
    let timestamp = Clock::get()?.unix_timestamp;

    msg!(
        "CAR ERROR → VIN: {}, Code: {}, Message: {}, Timestamp: {}",
        car.vin,
        error_code,
        message,
        timestamp
    );

    emit!(ErrorReported {
        vin: car.vin.clone(),
        timestamp,
        error_code,
        message,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ReportError<'info> {
    #[account(
        seeds = [b"car", car.vin.as_bytes()],
        bump = car.obd_bumps,
    )]
    pub car: Account<'info, Car>,

    #[account()]
    pub authority: Signer<'info>, // E.g. backend system or car device signer
}
