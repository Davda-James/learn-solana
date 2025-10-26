#![allow(deprecated)]
pub mod  states;
pub mod errors;
pub mod events;
pub mod instructions;

use instructions::*;
use anchor_lang::prelude::*;

declare_id!("EXmkeXZ86tCX3eHqwv9seGrKi3ngb34KcdMtpqz1Vk8T");

#[program]
pub mod spl_token {

    use super::*;

    pub fn initialize(ctx: Context<InitializeMint>, decimals: u8) -> Result<()> {
        initialize_mint(ctx, decimals)
    }

    pub fn init_token_account(ctx: Context<InitializeTokenAccount>, amount: u64) -> Result<()> {
        initialize_token_account(ctx, amount)
    }

    pub fn transfer(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        transfer_tokens(ctx, amount)
    }

    pub fn burn_tokens(ctx: Context<Burn>, amount: u64) -> Result<()> {
        burn(ctx, amount)
    }  

    pub fn freeze(ctx: Context<Freeze>) -> Result<()> {
        freeze_account(ctx)
    }
    pub fn unfreeze(ctx: Context<UnFreeze>) -> Result<()> {
        unfreeze_account(ctx)
    }

}


