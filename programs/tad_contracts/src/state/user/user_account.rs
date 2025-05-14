use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub wallet: Pubkey,
    pub email: String,
}

impl User {
    pub const LEN: usize = 8 + 32 + 4 + 64; // assuming email max 64 chars
}
