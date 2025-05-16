use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub points: u64,
    pub email: String,
}

impl User {
    pub const MAX_EMAIL_LEN: usize = 64;
    pub const LEN: usize = 8 + 32 + 8 + 4 + Self::MAX_EMAIL_LEN;
}
