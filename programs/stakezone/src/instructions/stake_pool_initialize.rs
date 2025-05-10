use anchor_lang::prelude::*;
use crate::state::pool::Pool;
use crate::errors::ErrorCode;
use anchor_spl::token::{Mint, Token};
use crate::state::RewardTier;

#[derive(Accounts)]
#[instruction(entry_fee: u64, reward_tiers: Vec<RewardTier>, start_time: i64, end_time: i64)]
pub struct InitializeStakePool<'info> {
    #[account(mut, signer)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Pool::MAX_SIZE,
        seeds = [b"pool", creator.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub fee_payer: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeStakePool<'info> {
    pub fn validate(&self, entry_fee: u64, reward_tiers: &[RewardTier], start_time: i64, end_time: i64) -> Result<()> {
        require!(entry_fee > 0, ErrorCode::InvalidEntryFee);
        require!(start_time < end_time, ErrorCode::InvalidTimeRange);
        require!(reward_tiers.len() <= Pool::MAX_REWARD_TIERS, ErrorCode::TooManyRewardTiers);
        let total_percentage: u8 = reward_tiers.iter().map(|tier| tier.percentage).sum();
        require!(total_percentage == 100, ErrorCode::InvalidRewardTierSum);
        Ok(())
    }
}

pub fn handler(
    ctx: Context<InitializeStakePool>,
    entry_fee: u64,
    reward_tiers: Vec<RewardTier>,
    start_time: i64,
    end_time: i64,
) -> Result<()> {
    // Validate input parameters
    ctx.accounts.validate(entry_fee, &reward_tiers, start_time, end_time)?;
    // Initialize pool
    let pool = &mut ctx.accounts.pool;
    pool.creator = ctx.accounts.creator.key();
    pool.entry_fee = entry_fee;
    pool.reward_tiers = reward_tiers;
    pool.start_time = start_time;
    pool.end_time = end_time;
    pool.participants = 0;
    pool.is_active = true;
    pool.usdc_mint = ctx.accounts.usdc_mint.key();
    pool.bump = ctx.bumps.pool;
    Ok(())
}