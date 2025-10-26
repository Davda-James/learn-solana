use anchor_lang::prelude::*;
use crate::states::TokenAccount;
use crate::errors::*;
use crate::events::*;


#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info,TokenAccount>,
    pub owner: Signer<'info>
}

pub fn transfer_tokens(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;
    let to = &mut ctx.accounts.to;

    require_keys_eq!(from.owner, ctx.accounts.owner.key(), ErrorType::Unauthorized);
    require_keys_eq!(from.mint, to.mint, ErrorType::InvalidMint);
    // cannot take transfer tokens if any one one is frozen 
    require!(!from.frozen && !to.frozen, ErrorType::FrozenAccount);
    
    from.amount = from.amount.checked_sub(amount).ok_or(error!(ErrorType::InsufficientTokens))?;
    to.amount = to.amount.checked_add(amount).ok_or(error!(ErrorType::ValueOverflowed))?;

    emit!(
        TokensTransferred {
            mint: from.mint,
            from: from.key(),
            to: to.key(),
            amount: amount,
        }
    );
    Ok(())
} 



