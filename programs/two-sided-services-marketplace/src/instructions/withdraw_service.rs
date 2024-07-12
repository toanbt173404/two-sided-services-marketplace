use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::helper::transfer_nft_from_pool_to_user;
use crate::CONFIG_SEED;
use crate::{
    error::ProgramErrorCode,
    states::{ConfigAccount, ServiceAccount},
};

#[derive(Accounts)]
pub struct WithdrawNFTService<'info> {
    #[account(mut)]
    pub vendor: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub config_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub nft_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = vendor,
        associated_token::mint = nft_mint,
        associated_token::authority = vendor
    )]
    pub vendor_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
     mut,
     constraint = nft_mint.key() == service_account.nft_mint,
     constraint = vendor.key() == service_account.current_vendor
  )]
    pub service_account: Account<'info, ServiceAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_service(ctx: Context<WithdrawNFTService>) -> Result<()> {
    let service_account = &ctx.accounts.service_account;

    //not support soulbound nft cause non transferable
    if service_account.is_soulbound {
        return Err(ProgramErrorCode::NotSupportSoulBound.into());
    }

    //send non soulbound NFT to current vendor
    let seeds = &CONFIG_SEED.as_bytes();
    let bump = ctx.accounts.config_account.bump;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    transfer_nft_from_pool_to_user(
        ctx.accounts.config_token_account.to_account_info(),
        ctx.accounts.vendor_token_account.to_account_info(),
        ctx.accounts.config_account.to_account_info(),
        ctx.accounts.nft_mint.clone(),
        ctx.accounts.token_program.to_account_info(),
        signer,
    )?;

    Ok(())
}
