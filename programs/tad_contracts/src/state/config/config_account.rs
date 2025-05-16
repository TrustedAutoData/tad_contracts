use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub vault: Pubkey,
}

impl Config {
    pub const LEN: usize = 8 + 32 + 32;
}
