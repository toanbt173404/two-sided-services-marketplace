use anchor_lang::prelude::*;

pub const ASK_SEED: &str = "ask";

#[account]
pub struct AskAccount {
    pub bump: u8,
    pub asker: Pubkey,
    pub nft_mint: Pubkey,
    pub ask_price: u64,
}

impl Space for AskAccount {
    const INIT_SPACE: usize = 8  // Account discriminator added by Anchor for each account
        + 1  // bump
        + 32 // asker
        + 32 // nft_mint
        + 8; // price
}
