use anchor_lang::prelude::*;

use crate::{states::*, errors::*, events::*};

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(
        has_one = freeze_authority
    )]  
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub freeze_authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>
} 

#[derive(Accounts)]
pub struct UnFreeze<'info> {
    #[account(
        has_one = freeze_authority
    )]
    pub mint: Account<'info,Mint>,
    #[account(mut)]
    pub freeze_authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>
    
}
pub fn freeze_account(ctx: Context<Freeze>) -> Result<()> {
    let token_account =  &mut ctx.accounts.token_account;
    // fail if already frozen
    require!(!token_account.frozen, ErrorType::AlreadyFrozen);
    token_account.frozen = true;
    emit!(
        AccountFreezed {
            mint: ctx.accounts.mint.key(),
            freeze_authority: ctx.accounts.freeze_authority.key(),
            freezed_account: token_account.key() 
        }
    );
    Ok(())
}

pub fn unfreeze_account(ctx: Context<UnFreeze>) -> Result<()> {
    let token_account = &mut ctx.accounts.token_account;
    require!(token_account.frozen, ErrorType::AlreadyUnFrozen);
    token_account.frozen = false;
    emit!(
        AccountUnFreezed {
            mint: ctx.accounts.mint.key(),
            freeze_authority: ctx.accounts.freeze_authority.key(),
            freezed_acccount: token_account.key()
        }
    );
    Ok(())
}
