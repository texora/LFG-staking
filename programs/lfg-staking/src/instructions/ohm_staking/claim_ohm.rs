use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct ClaimOhm<'info>{
    #[account(mut)]
    pub user: Signer<'info>
}

pub fn claim_ohm(ctx: Context<ClaimOhm>, amount: u64) -> Result<()>{
    
    Ok(())
}