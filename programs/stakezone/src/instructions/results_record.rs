use anchor_lang::prelude::*;
use crate::state::pool::Pool;
use crate::state::participation::Participation;
use crate::errors::ErrorCode;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(score: u16)]
pub struct RecordScore<'info> {
    #[account(
        mut,
        seeds = [b"participation", pool.key().as_ref(), participant.key().as_ref()],
        bump = participation.bump,
        has_one = pool,
        has_one = participant,
    )]
    pub participation: Account<'info, Participation>,
    #[account(
        seeds = [b"pool", pool.creator.as_ref()],
        bump = pool.bump,
        has_one = usdc_mint,
        constraint = pool.end_time < Clock::get()?.unix_timestamp, // Ensure pool end time has passed
    )]
    pub pool: Account<'info, Pool>,
    pub usdc_mint: Account<'info, Mint>, // Ensure the pool is using the correct USDC mint
    /// CHECK: Only the designated oracle should be able to sign this.
    #[account(signer)]
    pub oracle: AccountInfo<'info>,
    #[account(
        seeds = [b"oracle", pool.key().as_ref()], // Example: Oracle PDA tied to the pool
        bump,
        constraint = oracle.key() == oracle_account.key(),
    )]
    pub oracle_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RecordScore>,
    score: u16,
) -> Result<()> {
    let participation = &mut ctx.accounts.participation;
    participation.score = score;
    Ok(())
}