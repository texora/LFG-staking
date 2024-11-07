use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Token, Mint, Transfer, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::errors::ErrorCode;
use crate::{
    MAXIMUM_AGE,
    FWOG_MEME_COIN, BONK_MEME_COIN, POPCAT_MEME_COIN, REWARD_TOKEN_MINT_ADDRESS, 
    SOL_USD_FEED_ID, FWOG_MEME_COIN_FEED_ID, BONK_MEME_COIN_FEED_ID, POPCAT_MEME_COIN_FEED_ID
};

#[derive(Accounts)]
pub struct UnStakeMemeCoin<'info>{
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
        constraint=user_reward_ata.mint==*(reward_token_mint.to_account_info().key) && user_reward_ata.owner==*(user.to_account_info().key)
    )]
    user_reward_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=user_ata.mint==*(meme_coin_mint.to_account_info().key) && user_ata.owner==*(user.to_account_info().key)
    )]
    user_ata: Account<'info, TokenAccount>,

    price_update: Account<'info, PriceUpdateV2>,

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

pub fn unstake_meme_coin(ctx: Context<UnStakeMemeCoin>, amount: u64) -> Result<()>{
    require!(amount > 0, ErrorCode::WrongAmount);

    // recieve fee sol 5% for exit

    let price_update = &mut ctx.accounts.price_update;

    let sol_price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(SOL_USD_FEED_ID)?
    )?;

    if ctx.accounts.meme_coin_mint.key() == FWOG_MEME_COIN.parse::<Pubkey>().unwrap() {
        let meme_coin_price = price_update.get_price_no_older_than(
            &Clock::get()?,
            MAXIMUM_AGE,
            &get_feed_id_from_hex(FWOG_MEME_COIN_FEED_ID)?
        )?;

        let sol_fee_amount = ( ( meme_coin_price.price as u64 ) / ( sol_price.price as u64 ) ) * amount * 5 / 100; // exit: 5% fee.

        let cpi_ctx1 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer{
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.meme_coin_pda.to_account_info(),
            }
        );

        system_program::transfer(cpi_ctx1, sol_fee_amount);
    }
    else if ctx.accounts.meme_coin_mint.key() == BONK_MEME_COIN.parse::<Pubkey>().unwrap() {
        let meme_coin_price = price_update.get_price_no_older_than(
            &Clock::get()?,
            MAXIMUM_AGE,
            &get_feed_id_from_hex(BONK_MEME_COIN_FEED_ID)?
        )?;

        let sol_fee_amount = ( ( meme_coin_price.price as u64 ) / ( sol_price.price as u64 ) ) * amount * 5 / 100; // exit: 5% fee.

        let cpi_ctx1 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer{
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.meme_coin_pda.to_account_info(),
            }
        );

        system_program::transfer(cpi_ctx1, sol_fee_amount);
    }
    else if ctx.accounts.meme_coin_mint.key() == POPCAT_MEME_COIN_FEED_ID.parse::<Pubkey>().unwrap() {
        let meme_coin_price = price_update.get_price_no_older_than(
            &Clock::get()?,
            MAXIMUM_AGE,
            &get_feed_id_from_hex(POPCAT_MEME_COIN_FEED_ID)?
        )?;

        let sol_fee_amount = ( ( meme_coin_price.price as u64 ) / ( sol_price.price as u64 ) ) * amount * 5 / 100; // exit: 5% fee.

        let cpi_ctx1 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer{
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.meme_coin_pda.to_account_info(),
            }
        );

        system_program::transfer(cpi_ctx1, sol_fee_amount);
    }

    // unstake meme coin
    let meme_staking_user_info = &mut ctx.accounts.meme_staking_user_info;

    let cpi_accounts1 = Transfer {
        from: ctx.accounts.meme_coin_ata.to_account_info(),
        to: ctx.accounts.user_ata.to_account_info(),
        authority: ctx.accounts.meme_coin_pda.to_account_info()
    };
    
    let bump = ctx.bumps.meme_coin_pda;
    let seeds = &[b"meme_coin_pda".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx1 = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts1,
        signer
    );

    token::transfer(cpi_ctx1, meme_staking_user_info.amount);

    // calcurate reward token

    let clock = Clock::get()?;
    let reward = &mut 0;

    if meme_staking_user_info.amount > 0 {
        let slot = ( clock.slot - meme_staking_user_info.deposit_slot ) / ( 24 * 3600 );
        *reward = meme_staking_user_info.amount / 10 * slot;
    }

    // get reward token

    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_token_ata.to_account_info(),
        to: ctx.accounts.user_reward_ata.to_account_info(),
        authority: ctx.accounts.reward_token_pda.to_account_info()
    };
    
    let bump = ctx.bumps.reward_token_pda;
    let seeds = &[b"reward_token_pda".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer
    );

    token::transfer(cpi_ctx, *reward);

    meme_staking_user_info.amount = 0;

    meme_staking_user_info.deposit_slot = clock.slot;

    meme_staking_user_info.reward = 0;
    
    Ok(())
}