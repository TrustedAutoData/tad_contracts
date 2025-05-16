use anchor_lang::prelude::*;

#[account]
pub struct DealerReportData {
    pub report_id: u64,
    pub content_uri: String,
    pub is_owner_nft: bool,
}

impl DealerReportData {
    pub const MAX_URI_LENGTH: usize = 200;
    pub const MAX_LEN: usize = 8 + // discriminator
        8 + // report_id
        4 + Self::MAX_URI_LENGTH + 1;
}
