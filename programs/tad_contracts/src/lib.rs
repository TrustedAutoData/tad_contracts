use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("G9bxnJromWRrVNNP4WgY4u2x2E7tWJ317vjEaKxpqbJb");

#[program]
pub mod tad_contracts {
    use crate::instruction::AddUserPoints;

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

    pub fn register_service_attendance(
        ctx: Context<RegisterService>,
        report_id: u64,
        content_uri: String,
        report_type: String,
    ) -> Result<()> {
        register_service(ctx, report_id, content_uri, report_type)
    }

    pub fn get_report(
        ctx: Context<Report>,
        report_id: u64,
        content_uri: String,
        report_type: String,
    ) -> Result<()> {
        get_car_report(ctx, report_id, content_uri, report_type)
    }

    pub fn report_car_error(
        ctx: Context<ReportError>,
        error_code: u16,
        message: String,
    ) -> Result<()> {
        report_error(ctx, error_code, message)
    }
    pub fn add_user_points(ctx: Context<AddPointsForService>, points_to_add: u64) -> Result<()> {
        add_points_for_service(ctx, points_to_add)
    }
}
