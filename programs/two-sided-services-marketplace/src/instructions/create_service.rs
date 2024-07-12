use std::ops::DerefMut;

use anchor_lang::{
    prelude::*,
    solana_program::program::{invoke, invoke_signed},
    system_program,
};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token_2022::{
        self,
        spl_token_2022::{self, extension::ExtensionType, state::Mint},
    },
    token_interface::{
        non_transferable_mint_initialize, spl_token_2022::instruction::AuthorityType,
        spl_token_metadata_interface, NonTransferableMintInitialize, Token2022,
    },
};

use crate::{
    constant::METADATA_URI,
    error::ProgramErrorCode,
    states::{ConfigAccount, ServiceAccount, ServiceAgreement, CONFIG_SEED, SERVICE_SEED},
};

#[derive(Accounts)]
pub struct ListService<'info> {
    #[account(mut)]
    pub vendor: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub nft_mint: Signer<'info>,
    /// CHECK: We will create this one for the user
    #[account(mut)]
    pub config_token_account: AccountInfo<'info>,
    #[account(
      init_if_needed,
      payer = vendor,
      space = ServiceAccount::INIT_SPACE,
      seeds = [&SERVICE_SEED.as_bytes(), nft_mint.key().as_ref()],
      bump
  )]
    pub service_account: Account<'info, ServiceAccount>,

    pub token_program: Program<'info, Token2022>,

    pub rent: Sysvar<'info, Rent>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn create_service(
    ctx: Context<ListService>,
    is_soulbound: bool,
    agreements: Vec<ServiceAgreement>,
    price: u64,
) -> Result<()> {
    let service_account = ctx.accounts.service_account.deref_mut();

    service_account.bump = ctx.bumps.service_account;
    service_account.is_soulbound = is_soulbound;
    service_account.current_vendor = ctx.accounts.vendor.key();
    service_account.original_vendor = ctx.accounts.vendor.key();
    service_account.nft_mint = ctx.accounts.nft_mint.key();
    service_account.price = price;

    initialize_mint_and_metadata(&ctx, is_soulbound, agreements)?;

    Ok(())
}

pub fn initialize_mint_and_metadata(
    ctx: &Context<ListService>,
    is_soulbound: bool,
    agreements: Vec<ServiceAgreement>,
) -> Result<()> {
    let space =
        match ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer]) {
            Ok(space) => space,
            Err(_) => return err!(ProgramErrorCode::InvalidMintAccountSpace),
        };

    let meta_data_space = 250 + agreements.len() * 200;
    let lamports_required = (Rent::get()?).minimum_balance(space + meta_data_space);

    // Create Mint account
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.vendor.to_account_info(),
                to: ctx.accounts.nft_mint.to_account_info(),
            },
        ),
        lamports_required,
        space as u64,
        &ctx.accounts.token_program.key(),
    )?;

    // Assign the mint to the token program
    system_program::assign(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::Assign {
                account_to_assign: ctx.accounts.nft_mint.to_account_info(),
            },
        ),
        &token_2022::ID,
    )?;

    // Initialize the metadata pointer
    let init_meta_data_pointer_ix =
        match spl_token_2022::extension::metadata_pointer::instruction::initialize(
            &Token2022::id(),
            &ctx.accounts.nft_mint.key(),
            Some(ctx.accounts.config_account.key()),
            Some(ctx.accounts.nft_mint.key()),
        ) {
            Ok(ix) => ix,
            Err(_) => return err!(ProgramErrorCode::CantInitializeMetadataPointer),
        };

    invoke(
        &init_meta_data_pointer_ix,
        &[
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.config_account.to_account_info(),
        ],
    )?;

    // Initialize the mint cpi
    let mint_cpi_ix = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token_2022::InitializeMint2 {
            mint: ctx.accounts.nft_mint.to_account_info(),
        },
    );

    if is_soulbound {
        // Initialize the NonTransferable extension
        // This instruction must come before the instruction to initialize the mint data
        non_transferable_mint_initialize(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            NonTransferableMintInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
            },
        ))?;
    }

    token_2022::initialize_mint2(mint_cpi_ix, 0, &ctx.accounts.config_account.key(), None)?;

    // PDA for mint authority
    let seeds = &CONFIG_SEED.as_bytes();
    let bump = ctx.accounts.config_account.bump;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    let init_token_meta_data_ix = &spl_token_metadata_interface::instruction::initialize(
        &spl_token_2022::id(),
        ctx.accounts.nft_mint.key,
        ctx.accounts.config_account.to_account_info().key,
        ctx.accounts.nft_mint.key,
        ctx.accounts.config_account.to_account_info().key,
        "Two Sided Services Marketplace".to_string(),
        "TSSM".to_string(),
        METADATA_URI.to_string(),
    );

    invoke_signed(
        init_token_meta_data_ix,
        &[
            ctx.accounts.nft_mint.to_account_info().clone(),
            ctx.accounts.config_account.to_account_info().clone(),
        ],
        signer,
    )?;

    // Iterate over agreements and update metadata fields
    for agreement in agreements.iter() {
        invoke_signed(
            &spl_token_metadata_interface::instruction::update_field(
                &spl_token_2022::id(),
                ctx.accounts.nft_mint.key,
                ctx.accounts.config_account.to_account_info().key,
                spl_token_metadata_interface::state::Field::Key(agreement.title.clone()),
                agreement.details.clone(),
            ),
            &[
                ctx.accounts.nft_mint.to_account_info().clone(),
                ctx.accounts.config_account.to_account_info().clone(),
            ],
            signer,
        )?;
    }

    // Create the associated token account
    associated_token::create(CpiContext::new_with_signer(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.vendor.to_account_info(),
            associated_token: ctx.accounts.config_token_account.to_account_info(),
            authority: ctx.accounts.config_account.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
        signer
    ))?;

    // Mint one token to the associated token account
    token_2022::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::MintTo {
                mint: ctx.accounts.nft_mint.to_account_info(),
                to: ctx.accounts.config_token_account.to_account_info(),
                authority: ctx.accounts.config_account.to_account_info(),
            },
            signer,
        ),
        1,
    )?;

    // Freeze the mint authority to make it an NFT
    token_2022::set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::SetAuthority {
                current_authority: ctx.accounts.config_account.to_account_info(),
                account_or_mint: ctx.accounts.nft_mint.to_account_info(),
            },
            signer,
        ),
        AuthorityType::MintTokens,
        None,
    )?;

    Ok(())
}
