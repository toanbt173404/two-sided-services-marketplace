use anchor_lang::prelude::*;
pub mod constant;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("DRZPfcvLTK73RVEWyG3hqQ3BkDtbHeUDAn5Z6ScXHqF2");

#[program]
pub mod two_sided_services_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, royalty_fee_basis_points: u16) -> Result<()> {
        instructions::initialize(ctx, royalty_fee_basis_points)?;
        Ok(())
    }

    pub fn create_service(
        ctx: Context<ListService>,
        is_soulbound: bool,
        agreements: Vec<ServiceAgreement>,
        price: u64,
    ) -> Result<()> {
        instructions::create_service(ctx, is_soulbound, agreements, price)?;
        Ok(())
    }

    pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
        instructions::buy_service(ctx)?;
        Ok(())
    }

    pub fn ask_service(ctx: Context<AskService>, ask_price: u64) -> Result<()> {
        instructions::ask_service(ctx, ask_price)?;
        Ok(())
    }

    pub fn accept_ask(ctx: Context<AccpectAsk>) -> Result<()> {
        instructions::accept_ask(ctx)?;
        Ok(())
    }

    pub fn withdraw_service(ctx: Context<WithdrawNFTService>) -> Result<()> {
        instructions::withdraw_service(ctx)?;
        Ok(())
    }

    pub fn update_ask_price(ctx: Context<UpdateAskPrice>, new_ask_price: u64) -> Result<()> {
        instructions::update_ask_price(ctx, new_ask_price)?;
        Ok(())
    }

    pub fn update_service_price(ctx: Context<UpdateServicePrice>, new_service_price: u64) -> Result<()> {
        instructions::update_service_price(ctx, new_service_price)?;
        Ok(())
    }
}
