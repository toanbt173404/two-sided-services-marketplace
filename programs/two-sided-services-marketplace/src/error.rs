use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("The configuration account is already initialized.")]
    AlreadyInitialized,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Invalid Mint account space")]
    InvalidMintAccountSpace,
    #[msg("Cant initialize metadata_pointer")]
    CantInitializeMetadataPointer,
    #[msg("Operation resulted in an overflow.")]
    Overflow,
    #[msg("Attempted to divide by zero.")]
    DivideByZero,
    #[msg("Not support soul bound NFT")]
    NotSupportSoulBound,
    #[msg("Current vendor does not match the vendor key.")]
    InvalidCurrentVendor,
    #[msg("Original vendor does not match the vendor key.")]
    InvalidOriginalVendor,
    #[msg("NFT mint of the service account does not match the NFT mint of the ask account.")]
    InvalidNftMint,
}
