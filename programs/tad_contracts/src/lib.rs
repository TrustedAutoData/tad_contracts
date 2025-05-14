use anchor_lang::prelude::*;

pub mod errors;
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
}
