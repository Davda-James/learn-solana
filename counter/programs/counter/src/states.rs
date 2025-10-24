use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter { 
    pub value: u64,
    pub authority: Pubkey     
}  


