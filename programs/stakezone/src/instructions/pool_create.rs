use anchor_lang::prelude::*;
use crate::state::Pool;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(entry_fee: u64, reward_tiers: Vec<(u8, u8)>, start_time: i64, end_time: i64)]
pub struct CreatePool<'info> {
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
    pub usdc_mint: Account<'info, Mint>, // Add the USDC mint account
    #[account(mut)]
    pub fee_payer: SystemAccount<'info>, // Account to receive platform fees (in SOL)
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreatePool>,
    entry_fee: u64,
    reward_tiers: Vec<(u8, u8)>,
    start_time: i64,
    end_time: i64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.creator = ctx.accounts.creator.key();
    pool.entry_fee = entry_fee;
    pool.reward_tiers = reward_tiers;
    pool.start_time = start_time;
    pool.end_time = end_time;
    pool.participants = 0;
    pool.is_active = true;
    pool.usdc_mint = ctx.accounts.usdc_mint.key(); // Store the USDC mint address
    pool.bump = *ctx.bumps.get("pool").unwrap();

    Ok(())
}