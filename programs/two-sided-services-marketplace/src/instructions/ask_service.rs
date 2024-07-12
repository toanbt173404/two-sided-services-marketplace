use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{
    helper::send_lamports,
    states::{ConfigAccount, ServiceAccount},
    AskAccount, ASK_SEED,
};

#[derive(Accounts)]
pub struct AskService<'info> {
    #[account(mut)]
    pub asker: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    /// CHECK: Mint account.
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,
    #[account(
      init_if_needed,
      payer = asker,
      space = ServiceAccount::INIT_SPACE,
      seeds = [&ASK_SEED.as_bytes(), nft_mint.key().as_ref()],
      bump
  )]
    pub ask_account: Account<'info, AskAccount>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct AskServiceEvent {
    pub asker: Pubkey,
    pub nft_mint: Pubkey,
    pub ask_price: u64,
}

pub fn ask_service(ctx: Context<AskService>, ask_price: u64) -> Result<()> {
    let ask_account = ctx.accounts.ask_account.deref_mut();

    ask_account.bump = ctx.bumps.ask_account;
    ask_account.ask_price = ask_price;
    ask_account.asker = ctx.accounts.asker.key();
    ask_account.nft_mint = ctx.accounts.nft_mint.key();

    //send lamports to program
    send_lamports(
        ctx.accounts.asker.to_account_info(),
        ctx.accounts.ask_account.to_account_info(),
        ask_price,
    )?;

    emit!(AskServiceEvent {
        asker: ctx.accounts.asker.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        ask_price,
    });

    Ok(())
}
