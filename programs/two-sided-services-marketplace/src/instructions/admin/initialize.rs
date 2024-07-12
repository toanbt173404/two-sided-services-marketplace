use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{
    error::ProgramErrorCode,
    states::{ConfigAccount, CONFIG_SEED},
};

#[event]
pub struct InitializeEvent {
    pub admin: Pubkey,
    pub royalty_fee_basis_points: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = ConfigAccount::INIT_SPACE,
        seeds = [&CONFIG_SEED.as_bytes()],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, royalty_fee_basis_points: u16) -> Result<()> {
    let config_account = ctx.accounts.config_account.deref_mut();

    if config_account.is_initialized {
        return Err(ProgramErrorCode::AlreadyInitialized.into());
    }

    config_account.bump = ctx.bumps.config_account;
    config_account.is_initialized = true;
    config_account.admin = ctx.accounts.admin.key();
    config_account.royalty_fee_basis_points = royalty_fee_basis_points;

    emit!(InitializeEvent {
        admin: ctx.accounts.admin.key(),
        royalty_fee_basis_points,
    });

    Ok(())
}
