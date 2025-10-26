use anchor_lang::prelude::*;
use crate::states::*;
use crate::events::*;
use crate::errors::*;


pub fn initialize_mint(ctx : Context<InitializeMint>, decimals: u8) -> Result<()> {
    let mint = &mut ctx.accounts.mint;
    mint.mint_authority = ctx.accounts.owner.key();
    mint.decimals = decimals;
    mint.freeze_authority = ctx.accounts.owner.key();
    mint.supply = 0;
    emit!(MintInitialized {
            mint: mint.key(),
            mint_authority: mint.mint_authority,
            supply: mint.supply,
            decimals: decimals,
            freeze_authority: mint.freeze_authority
    });
    Ok(())
}

pub fn initialize_token_account(ctx: Context<InitializeTokenAccount>, amount: u64) -> Result<()> {
    let token_accnt = &mut ctx.accounts.token_account;
    token_accnt.owner = ctx.accounts.owner.key();
    token_accnt.mint = ctx.accounts.mint.key();
    token_accnt.amount = amount;
    token_accnt.frozen = false;
    let mint = &mut ctx.accounts.mint;
    mint.supply = mint.supply.checked_add(amount).ok_or(error!(ErrorType::TokenSupplyOverflowed))?;
    msg!("{}",mint.supply);
    emit!(
        TokenAccountCreated {
            mint: mint.key(),
            owner: token_accnt.owner.key(),
            amount: amount,
            frozen: token_accnt.frozen
        }
    );
    
    Ok(())
}


#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Mint::INIT_SPACE
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + TokenAccount::INIT_SPACE
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info,System>
}

