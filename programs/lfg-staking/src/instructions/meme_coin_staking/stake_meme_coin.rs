use anchor_lang::prelude::*;

use anchor_spl::token::{self, Token, Mint, Transfer, TokenAccount};

use crate::errors::ErrorCode;
use crate::{FWOG_MEME_COIN, BONK_MEME_COIN, POPCAT_MEME_COIN, REWARD_TOKEN_MINT_ADDRESS};

#[derive(Accounts)]
pub struct StakeMemeCoin<'info>{
    #[account(
        mut,
        seeds=[b"meme_coin_pda"],
        bump
    )]
    meme_coin_pda: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"meme_coin_ata", meme_coin_mint.key().as_ref()],
        token::mint=meme_coin_mint,
        token::authority=meme_coin_pda,
        bump
    )]
    meme_coin_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=( meme_coin_mint.key() == FWOG_MEME_COIN.parse::<Pubkey>().unwrap() || 
        meme_coin_mint.key() == BONK_MEME_COIN.parse::<Pubkey>().unwrap() || 
        meme_coin_mint.key() == POPCAT_MEME_COIN.parse::<Pubkey>().unwrap() ) @ ErrorCode::NotAllowMemeCoin
    )]
    meme_coin_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[b"reward_token_pda"],
        bump
    )]
    reward_token_pda: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"reward_token_ata", ],
        token::mint=reward_token_mint,
        token::authority=reward_token_pda,
        bump
    )]
    reward_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        address=REWARD_TOKEN_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    reward_token_mint: Account<'info, Mint>,

    #[account(mut)]
    meme_staking_user_info: Account<'info, MemeStakingUserInfo>,

    #[account(
        mut,
        constraint=user_ata.mint==*(meme_coin_mint.to_account_info().key) && user_ata.owner==*(user.to_account_info().key)
    )]
    user_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    rent: Sysvar<'info, Rent>

}

#[account]
pub struct MemeStakingUserInfo{
    pub amount: u64,
    pub deposit_slot: u64,
    pub reward: u64
}

pub fn stake_meme_coin(ctx: Context<StakeMemeCoin>, amount: u64) -> Result<()>{
    require!(amount > 0, ErrorCode::WrongAmount);

    let meme_staking_user_info = &mut ctx.accounts.meme_staking_user_info;

    let clock = Clock::get()?;
    let reward = &mut 0;

    if meme_staking_user_info.amount > 0 {
        let slot = ( clock.slot - meme_staking_user_info.deposit_slot ) / ( 24 * 3600 );
        *reward = meme_staking_user_info.amount / 10 * slot;
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.user_ata.to_account_info(),
        to: ctx.accounts.reward_token_ata.to_account_info(),
        authority: ctx.accounts.user.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount);

    meme_staking_user_info.amount += amount;

    meme_staking_user_info.deposit_slot = clock.slot;

    meme_staking_user_info.reward = *reward;
    
    Ok(())
}