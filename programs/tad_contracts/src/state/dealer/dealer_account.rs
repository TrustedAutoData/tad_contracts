use anchor_lang::prelude::*;

#[account]
pub struct Dealer {
    pub authority: Pubkey,
    pub name: String,
}

impl Dealer {
    pub const LEN: usize = 8 + 32 + 4 + 64; // max name = 64 chars
}
