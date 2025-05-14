use anchor_lang::prelude::*;

#[account]
pub struct Car {
    pub vin: String,
    pub owner: Pubkey,
    pub dealer: Pubkey,
}

impl Car {
    pub const LEN: usize = 8 + 4 + 64 + 32 + 32; // max VIN = 64
}
