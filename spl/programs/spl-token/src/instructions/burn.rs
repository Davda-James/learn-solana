use anchor_lang::prelude::*;

use crate::{events::*, errors::ErrorType, states::{Mint, TokenAccount}};


#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub owner: Signer<'info>
}

pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
    let mint = &mut ctx.accounts.mint;
    let from = &mut ctx.accounts.from;
    
    require_keys_eq!(from.owner, ctx.accounts.owner.key(), ErrorType::Unauthorized);
    // ensure token account belongs to this mint
    require_keys_eq!(from.mint, mint.key(), ErrorType::InvalidMint);
    // cannot burn from a frozen account
    require!(!from.frozen, ErrorType::FrozenAccount);
    msg!("{} and {} and {}", from.amount,mint.supply,  amount);
    from.amount = from.amount.checked_sub(amount).ok_or(error!(ErrorType::InsufficientTokens))?;
    mint.supply = mint.supply.checked_sub(amount).ok_or(error!(ErrorType::InsufficientTokens))?;
    
    emit!(
        TokensBurnt {
            mint: from.mint.key(),
            owner: from.owner.key(),
            amount: amount
        }
    );
    Ok(())
}

