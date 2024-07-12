use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{
    constant::MAX_FEE_BASIS_POINTS,
    error::ProgramErrorCode,
    helper::send_lamports,
    states::{ConfigAccount, ServiceAccount},
};

#[derive(Accounts)]
pub struct BuyService<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: This account is the current vendor and is used to send lamports.
    #[account(mut)]
    pub current_vendor: AccountInfo<'info>,
    /// CHECK: This account is the original vendor and is used to send lamports.
    #[account(mut)]
    pub original_vendor: AccountInfo<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(
        mut,
        constraint = service_account.original_vendor == original_vendor.key(),
        constraint = service_account.current_vendor == current_vendor.key()
    )]
    pub service_account: Account<'info, ServiceAccount>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct BuyServiceEvent {
    pub buyer: Pubkey,
    pub current_vendor: Pubkey,
    pub original_vendor: Pubkey,
    pub price: u64,
    pub royalty_amount: u64,
    pub remaining_amount: u64,
}

pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
    let service_account = ctx.accounts.service_account.deref_mut();
    let config_account = &ctx.accounts.config_account;

    let price = service_account.price;
    let mut royalty_amount = 0;
    let mut remaining_amount = price;

    // If the current vendor is not the original vendor, proceed with royalties
    let current_vendor = service_account.current_vendor;

    if current_vendor != service_account.original_vendor {
        let royalty_fee_basis_points = config_account.royalty_fee_basis_points as u128;

        royalty_amount = (price as u128)
            .checked_mul(royalty_fee_basis_points)
            .ok_or(ProgramErrorCode::Overflow)?
            .checked_div(MAX_FEE_BASIS_POINTS as u128)
            .ok_or(ProgramErrorCode::DivideByZero)? as u64;
        remaining_amount = price
            .checked_sub(royalty_amount)
            .ok_or(ProgramErrorCode::Overflow)?;

        // Send royalties to the original vendor
        send_lamports(
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.original_vendor.to_account_info(),
            royalty_amount,
        )?;

        // Send the remaining amount to the current vendor
        send_lamports(
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.current_vendor.to_account_info(),
            remaining_amount,
        )?;
    } else {
        send_lamports(
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.current_vendor.to_account_info(),
            price,
        )?;
    }

    // Update the new vendor
    service_account.current_vendor = ctx.accounts.buyer.key();

    // Emit the event
    emit!(BuyServiceEvent {
        buyer: ctx.accounts.buyer.key(),
        current_vendor: ctx.accounts.current_vendor.key(),
        original_vendor: ctx.accounts.original_vendor.key(),
        price,
        royalty_amount,
        remaining_amount,
    });

    Ok(())
}
