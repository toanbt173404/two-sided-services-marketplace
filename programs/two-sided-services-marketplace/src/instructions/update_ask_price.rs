use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{error::ProgramErrorCode, helper::send_lamports, AskAccount};

#[derive(Accounts)]
pub struct UpdateAskPrice<'info> {
    #[account(mut)]
    pub asker: Signer<'info>,
    #[account(
        mut,
        constraint = ask_account.asker == asker.key()
    )]
    pub ask_account: Account<'info, AskAccount>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct UpdateAskPriceEvent {
    pub asker: Pubkey,
    pub nft_mint: Pubkey,
    pub old_ask_price: u64,
    pub new_ask_price: u64,
}

pub fn update_ask_price(ctx: Context<UpdateAskPrice>, new_ask_price: u64) -> Result<()> {
    let ask_account = &ctx.accounts.ask_account;

    let old_ask_price = ask_account.ask_price;

    // Determine the difference in price
    if new_ask_price > old_ask_price {
        // If the new price is higher, transfer additional lamports from the asker to the ask account
        let additional_lamports = new_ask_price
            .checked_sub(old_ask_price)
            .ok_or(ProgramErrorCode::Overflow)?;
        send_lamports(
            ctx.accounts.asker.to_account_info(),
            ctx.accounts.ask_account.to_account_info(),
            additional_lamports,
        )?;
    } else if new_ask_price < old_ask_price {
        // If the new price is lower, refund the difference from the ask account to the asker
        let refund_lamports = old_ask_price
            .checked_sub(new_ask_price)
            .ok_or(ProgramErrorCode::Overflow)?;
        **ctx
            .accounts
            .ask_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= refund_lamports;
        **ctx
            .accounts
            .asker
            .to_account_info()
            .try_borrow_mut_lamports()? += refund_lamports;
    }

    let ask_account = ctx.accounts.ask_account.deref_mut();
    ask_account.ask_price = new_ask_price;
    emit!(UpdateAskPriceEvent {
        asker: ctx.accounts.asker.key(),
        nft_mint: ask_account.nft_mint,
        old_ask_price,
        new_ask_price,
    });

    Ok(())
}
