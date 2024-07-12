use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{error::ProgramErrorCode, ServiceAccount};

#[derive(Accounts)]
pub struct UpdateServicePrice<'info> {
    #[account(mut)]
    pub vendor: Signer<'info>,
    #[account(
        mut,
        constraint = service_account.current_vendor == vendor.key() @ProgramErrorCode::Unauthorized
    )]
    pub service_account: Account<'info, ServiceAccount>,
}

#[event]
pub struct UpdateServicePriceEvent {
    pub vendor: Pubkey,
    pub nft_mint: Pubkey,
    pub old_price: u64,
    pub new_price: u64,
}

pub fn update_service_price(ctx: Context<UpdateServicePrice>, new_price: u64) -> Result<()> {
    let service_account = ctx.accounts.service_account.deref_mut();

    let old_price = service_account.price;
    service_account.price = new_price;

    emit!(UpdateServicePriceEvent {
        vendor: ctx.accounts.vendor.key(),
        nft_mint: service_account.nft_mint,
        old_price,
        new_price,
    });

    Ok(())
}
