#![allow(deprecated)]

mod states;
mod errors;

use states::*;
use errors::*;

use anchor_lang::prelude::*;


declare_id!("4hPdrNjok2KhTXbdjMBDDygmAam5aVVAJXBoWsjeuwUk");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.user.key();
        counter.value = 0;
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<u64> {
        let counter = &mut ctx.accounts.counter;
        counter.value += 1;
        Ok(counter.value)
    }
    pub fn decrement(ctx: Context<Decrement>) -> Result<u64> {
        let counter = &mut ctx.accounts.counter;
        counter.value -= 1;
        Ok(counter.value)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        has_one = authority @ ErrorType::UnAuthorized
    )]
    pub counter: Account<'info, Counter>,       
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(
        mut,
        has_one = authority @ ErrorType::UnAuthorized
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>
}


