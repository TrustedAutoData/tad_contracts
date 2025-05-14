use anchor_lang::prelude::*;
use anchor_spl::metadata::Metadata;
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair},
};

pub fn register_service(
    ctx: Context<RegisterService>,
    report_id: u64,
    report_name: String,
    content_uri: String,
    organization_name: String,
) -> Result<()> {
    let creator = ctx.accounts.creator.key();

    // Create service report NFT with attributes
    let attributes = Attributes {
        attribute_list: vec![
            Attribute {
                key: "report_id".to_string(),
                value: report_id.to_string(),
            },
            Attribute {
                key: "organization_name".to_string(),
                value: organization_name,
            },
            Attribute {
                key: "report_name".to_string(),
                value: report_name.clone(),
            },
        ],
    };

    let attributes_plugin = PluginAuthorityPair {
        plugin: Plugin::Attributes(attributes),
        authority: Some(PluginAuthority::Address { address: creator }),
    };

    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.owner_nft)
        .authority(Some(&creator))
        .payer(&ctx.accounts.creator)
        .owner(Some(&ctx.accounts.creator))
        .system_program(&ctx.accounts.system_program)
        .name(format!("Service Report: {}", report_name))
        .uri(content_uri.clone())
        .plugins(vec![
            attributes_plugin,
            PluginAuthorityPair {
                plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
                authority: None,
            },
        ])
        .invoke()?;

    let report_data = &mut ctx.accounts.report_data;
    report_data.report_id = report_id;
    report_data.content_uri = content_uri;
    report_data.is_owner_nft = true;
    report_data.shared_with = None;

    Ok(())
}

#[derive(Accounts)]
#[instruction(report_id: u64, content_uri: String, organization_name: String, report_name: String)]
pub struct RegisterService<'info> {
    #[account(
        init,
        payer = creator,
        space = ReportData::MAX_SIZE,
        seeds = [b"report_data", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report_data: Account<'info, ReportData>,

    /// CHECK: Initialized by Metaplex Core
    #[account(mut)]
    pub owner_nft: Signer<'info>,

    /// CHECK: Must match the signer who mints
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(address = mpl_token_metadata::ID)]
    pub mpl_token_metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,

    /// CHECK: Core CPI
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}
