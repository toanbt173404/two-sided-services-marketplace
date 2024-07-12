use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "config";
#[account]
pub struct ConfigAccount {
  pub bump: u8,
  pub is_initialized: bool,
  pub admin: Pubkey,
  pub royalty_fee_basis_points: u16
}

impl Space for ConfigAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1 //is_initialized
        + 32 // admin
        + 2; //royalty_fee_basis_points
}