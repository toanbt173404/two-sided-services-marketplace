use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{
    constant::MAX_FEE_BASIS_POINTS, error::ProgramErrorCode, states::{ConfigAccount, ServiceAccount}, AskAccount
};

#[derive(Accounts)]
pub struct AccpectAsk<'info> {
    #[account(mut)]
    pub vendor: Signer<'info>,
    /// CHECK: This account is the asker.
    #[account(mut)]
    pub asker: AccountInfo<'info>,
    /// CHECK: This account is the original vendor and is used to send lamports.
    #[account(mut)]
    pub original_vendor: AccountInfo<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub ask_account: Account<'info, AskAccount>,
    #[account(
        mut, 
        constraint = service_account.current_vendor == vendor.key() @ProgramErrorCode::InvalidCurrentVendor,
        constraint = service_account.original_vendor == original_vendor.key() @ProgramErrorCode::InvalidOriginalVendor,
        constraint = service_account.nft_mint == ask_account.nft_mint @ProgramErrorCode::InvalidNftMint
    )]
    pub service_account: Account<'info, ServiceAccount>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn accept_ask(ctx: Context<AccpectAsk>) -> Result<()> {
  let ask_account = ctx.accounts.ask_account.deref_mut();
  let config_account = &ctx.accounts.config_account;
  let service_account = ctx.accounts.service_account.deref_mut();

  let ask_price = ask_account.ask_price;
  let remaining_amount;

  // If the vendor is different from the original vendor, calculate and transfer the royalty fee
  if ctx.accounts.vendor.key() != ctx.accounts.original_vendor.key() {
      let royalty_fee_basis_points = config_account.royalty_fee_basis_points as u128;

      // Calculate the royalty fee
      let royalty_amount = (ask_price as u128)
          .checked_mul(royalty_fee_basis_points)
          .ok_or(ProgramErrorCode::Overflow)?
          .checked_div(MAX_FEE_BASIS_POINTS.into())  // Assuming basis points are out of 10,000
          .ok_or(ProgramErrorCode::DivideByZero)? as u64;

      remaining_amount = ask_price
          .checked_sub(royalty_amount)
          .ok_or(ProgramErrorCode::Overflow)?;

      // Transfer royalty fee to the original vendor
      **ctx.accounts.ask_account.to_account_info().try_borrow_mut_lamports()? -= royalty_amount;
      **ctx.accounts.original_vendor.to_account_info().try_borrow_mut_lamports()? += royalty_amount;
  } else {
      remaining_amount = ask_price;
  }

  // Transfer the remaining amount to the current vendor
  **ctx.accounts.ask_account.to_account_info().try_borrow_mut_lamports()? -= remaining_amount;
  **ctx.accounts.vendor.to_account_info().try_borrow_mut_lamports()? += remaining_amount;

  // Update the current vendor to the asker
  service_account.current_vendor = ctx.accounts.ask_account.asker;

  Ok(())
}