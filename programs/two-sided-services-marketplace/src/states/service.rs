use anchor_lang::prelude::*;

pub const SERVICE_SEED: &str = "service";

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ServiceAgreement {
    pub title: String,
    pub details: String,
}

#[account]
pub struct ServiceAccount {
    pub bump: u8,
    pub is_soulbound: bool,
    pub original_vendor: Pubkey,
    pub current_vendor: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
}

impl Space for ServiceAccount {
    const INIT_SPACE: usize = 8  // Account discriminator added by Anchor for each account
        + 1  // bump
        + 1  // is_soulbound
        + 32 // original_vendor
        + 32 // current_vendor
        + 32 // nft_mint
        + 8; // price
}
