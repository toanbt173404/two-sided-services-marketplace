use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{error::ProgramErrorCode, states::ConfigAccount};

#[derive(Accounts)]
pub struct UpdateRoyalty<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
}

pub fn update_royalty(
    ctx: Context<UpdateRoyalty>,
    new_royalty_fee_basis_points: u16,
) -> Result<()> {
    let config_account = ctx.accounts.config_account.deref_mut();

    // Ensure the caller is the admin
    if ctx.accounts.admin.key() != config_account.admin {
        return Err(ProgramErrorCode::Unauthorized.into());
    }

    // Update the royalty fee basis points
    config_account.royalty_fee_basis_points = new_royalty_fee_basis_points;

    Ok(())
}
