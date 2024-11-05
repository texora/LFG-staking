use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct ClaimOhm<'info>{
    /// CHECK
    #[account(
        mut,
        seeds=[b"token_vault_owner_pda"],
        bump
    )]
    staking_owner: AccountInfo<'info>

    // #[account(
    //     mut,
    //     seeds=
    // )]
}

pub fn claim_ohm(ctx: Context<ClaimOhm>, amount: u64) -> Result<()>{
    
    Ok(())
}