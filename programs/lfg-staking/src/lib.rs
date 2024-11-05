pub mod instructions;
pub mod errors;
pub mod constants;

use anchor_lang::prelude::*;

use instructions::*;
use constants::*;

declare_id!("HmXTaac3Cw3xvpfnkMC1nra8QZ1pA2W4c5PQ8HyozHKx");

#[program]
pub mod lfg_staking {
    use super::*;

    // ohm staking initialize
    pub fn ohm_initialize(ctx: Context<OhmInitialize>) -> Result<()> {
        msg!("Instruction: Ohm token Staking Initialize");
        instructions::ohm_initialize(ctx)
    }

    // stake ohm token
    pub fn stake_ohm(ctx: Context<StakeOhm>, amount: u64) -> Result<()> {
        msg!("Insruction: Stake ohm");
        instructions::stake_ohm(ctx, amount)
    }

    // claim ohm token
    pub fn claim_ohm(ctx: Context<ClaimOhm>, amount: u64) -> Result<()> {
        msg!("Instruction: Claim ohm");
        instructions::claim_ohm(ctx, amount)
    }
}
