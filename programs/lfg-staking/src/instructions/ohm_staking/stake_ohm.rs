use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct StakeOhm<'info>{
    #[account(mut)]
    pub user: Signer<'info>
}

pub fn stake_ohm(ctx: Context<StakeOhm>, amount: u64) -> Result<()>{
    Ok(())
}