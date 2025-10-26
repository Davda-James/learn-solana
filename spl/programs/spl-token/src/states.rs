use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Mint {
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub freeze_authority: Pubkey
}

#[account]
#[derive(InitSpace)]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub frozen: bool
}




