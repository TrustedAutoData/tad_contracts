use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("3AUUVkXFjfxxTDk7TpZg3VDuagqsihTrjmjka9HchDQc");

#[program]
pub mod tad_contracts {
    use super::*;

    pub fn initialize_car(ctx: Context<InitCar>, vin: String) -> Result<()> {
        init_car(ctx, vin)
    }

    pub fn initialize_config(ctx: Context<InitConfig>) -> Result<()> {
        init_config(ctx)
    }

    pub fn initialize_dealer(ctx: Context<InitDealer>, name: String) -> Result<()> {
        init_dealer(ctx, name)
    }

    pub fn initialize_user(ctx: Context<InitUser>, email: String) -> Result<()> {
        init_user(ctx, email)
    }

    pub fn register_car_km(ctx: Context<RegisterKm>, new_km: u64) -> Result<()> {
        register_km(ctx, new_km)
    }

    pub fn report_car_error(
        ctx: Context<ReportError>,
        error_code: u16,
        message: String,
    ) -> Result<()> {
        report_error(ctx, error_code, message)
    }
}
