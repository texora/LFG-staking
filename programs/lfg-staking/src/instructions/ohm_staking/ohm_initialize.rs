use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct OhmInitialize<'info>{
    #[account(mut)]
    pub admin: Signer<'info>
    
}

pub fn ohm_initialize(ctx: Context<OhmInitialize>) -> Result<()>{
    Ok(())
}