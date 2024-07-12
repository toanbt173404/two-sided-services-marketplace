use anchor_lang::prelude::*;
use anchor_spl::{token_2022, token_interface::Mint};

pub fn send_lamports<'a>(from: AccountInfo<'a>, to: AccountInfo<'a>, amount: u64) -> Result<()> {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &from.key(),
        &to.key(),
        amount.into(),
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[from.to_account_info(), to.to_account_info()],
    )
    .map_err(|err| err.into())
}

pub fn transfer_nft_from_pool_to_user<'info>(
    from_pool: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    token_program_2022: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let token_program_info = token_program_2022.to_account_info();
    let from_pool_info: AccountInfo = from_pool.to_account_info();

    token_2022::transfer_checked(
        CpiContext::new_with_signer(
            token_program_info,
            token_2022::TransferChecked {
                from: from_pool_info,
                to: to.to_account_info(),
                authority: authority.to_account_info(),
                mint: mint.to_account_info(),
            },
            signer_seeds,
        ),
        1,
        mint.decimals,
    )
}
