use anchor_lang::prelude::*;

#[event]
pub struct MintInitialized {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub freeze_authority: Pubkey
}

#[event]
pub struct TokenAccountCreated {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub frozen: bool
}

#[event]
pub struct TokensTransferred {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey, 
    pub amount: u64,
}

#[event]
pub struct TokensBurnt {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct AccountFreezed {
    pub mint: Pubkey,
    pub freeze_authority: Pubkey,
    pub freezed_account: Pubkey
}

#[event]
pub struct AccountUnFreezed {
    pub mint: Pubkey,
    pub freeze_authority: Pubkey,
    pub freezed_acccount: Pubkey
}