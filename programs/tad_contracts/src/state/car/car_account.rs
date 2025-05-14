use anchor_lang::prelude::*;

#[account]
pub struct Car {
    pub vin: String,
    pub owner: Pubkey,
    pub dealer: Pubkey,
    pub total_km: u64,
    pub service_count: u32,
    pub last_service_timestamp: i64,
    pub obd_bumps: u8,
}

impl Car {
    pub const MAX_VIN_LENGTH: usize = 64;

    pub const LEN: usize = 8 + 4 + Self::MAX_VIN_LENGTH + 32 + 32 + 8 + 4 + 8 + 1;
}
