use crate::state::{car::Car, Config, DealerReportData};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::metadata::{mpl_token_metadata, Metadata};
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair},
};

pub fn get_car_report(
    ctx: Context<Report>,
    report_id: u64,
    content_uri: String,
    report_type: String,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault.clone(),
            },
        ),
        25 * 10u64.pow(7 as u32),
    )?;

    let creator = ctx.accounts.creator.key();
    let car = &ctx.accounts.car;

    let attributes = Attributes {
        attribute_list: vec![
            Attribute {
                key: "vin".to_string(),
                value: car.vin.clone(),
            },
            Attribute {
                key: "report_type".to_string(),
                value: report_type.clone(),
            },
            Attribute {
                key: "total_km".to_string(),
                value: car.total_km.to_string(),
            },
        ],
    };

    let attributes_plugin = PluginAuthorityPair {
        plugin: Plugin::Attributes(attributes),
        authority: Some(PluginAuthority::Address { address: creator }),
    };

    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.owner_nft)
        .authority(Some(&ctx.accounts.creator))
        .payer(&ctx.accounts.user)
        .owner(Some(&ctx.accounts.user))
        .system_program(&ctx.accounts.system_program)
        .name(format!("Car report {}", car.vin))
        .uri(content_uri.clone())
        .plugins(vec![
            attributes_plugin,
            PluginAuthorityPair {
                plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
                authority: None,
            },
        ])
        .invoke()?;

    // Save metadata
    let report_data = &mut ctx.accounts.dealer_report_data;
    report_data.report_id = report_id;
    report_data.content_uri = content_uri;
    report_data.is_owner_nft = true;
    report_data.report_nft = ctx.accounts.owner_nft.key();

    Ok(())
}
#[derive(Accounts)]
#[instruction(report_id: u64, content_uri: String, report_type: String)]
pub struct Report<'info> {
    #[account(
        seeds = [b"car", car.vin.as_bytes()],
        bump = car.obd_bumps,
    )]
    pub car: Account<'info, Car>,

    #[account(
        init,
        payer = creator,
        space = DealerReportData::MAX_LEN,
        seeds = [b"dealer_report_data", car.key().as_ref(), report_id.to_le_bytes().as_ref()],
        bump
    )]
    pub dealer_report_data: Account<'info, DealerReportData>,

    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: Metaplex Core NFT address
    #[account(mut)]
    pub owner_nft: Signer<'info>,

    /// Signer creating the NFT (e.g., dealership)
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = config.vault)]
    pub vault: AccountInfo<'info>,

    #[account(address = mpl_token_metadata::ID)]
    pub mpl_token_metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,

    /// CHECK: Core CPI
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}
