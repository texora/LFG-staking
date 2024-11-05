use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BuyOhmWithSol<'info>{
    #[account(mut)]
    user: Signer<'info>
}

pub fn buy_ohm_with_sol(ctx: Context<BuyOhmWithSol>, amount: u64) -> Result<()>{
    Ok(())
}