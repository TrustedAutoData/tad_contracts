use anchor_lang::prelude::*;

#[event]
pub struct KmRegistered {
    pub vin: String,
    pub new_km: u64,
    pub updated_total: u64,
}

#[event]
pub struct ErrorReported {
    pub vin: String,
    pub timestamp: i64,
    pub error_code: u16,
    pub message: String,
}
