use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Mint};
use crate::state::{Pool, Participation};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(fpl_team_id: u32)]
pub struct JoinPool<'info> {
    #[account(mut, signer)]
    pub participant: Signer<'info>,
    #[account(
        mut,
        seeds = [b"pool", pool.creator.as_ref()],
        bump = pool.bump,
        has_one = usdc_mint,
        has_one = fee_payer,
        constraint = pool.is_active && Clock::get()?.unix_timestamp >= pool.start_time && Clock::get()?.unix_timestamp < pool.end_time,
    )]
    pub pool: Account<'info, Pool>,
    pub usdc_mint: Account<'info, Mint>, // Ensure the pool is using the correct USDC mint
    #[account(
        init,
        payer = participant,
        space = 8 + Participation::MAX_SIZE,
        seeds = [b"participation", pool.key().as_ref(), participant.key().as_ref()],
        bump,
        constraint = !Participation::did_participate(&ctx.program_id, &pool.key(), &participant.key())?,
    )]
    pub participation: Account<'info, Participation>,
    #[account(mut, token::mint = usdc_mint, token::owner = participant)]
    pub participant_usdc_account: Account<'info, TokenAccount>, // User's USDC account
    #[account(
        init_if_needed,
        payer = participant,
        space = 8 + TokenAccount::LEN,
        seeds = [b"pool_usdc", pool.key().as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = reward_authority, // Program or designated authority
    )]
    pub pool_usdc_account: Account<'info, TokenAccount>, // Pool's USDC escrow account
    /// CHECK: Authority to sign for the pool's USDC account (can be program or designated PDA)
    pub reward_authority: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub fee_payer: SystemAccount<'info>,
}

impl<'info> JoinPool<'info> {
    fn transfer_entry_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.participant_usdc_account.to_account_info(),
            to: self.pool_usdc_account.to_account_info(),
            authority: self.participant.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(
    ctx: Context<JoinPool>,
    fpl_team_id: u32,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participation = &mut ctx.accounts.participation;

    token::transfer(ctx.accounts.transfer_entry_fee(), pool.entry_fee)?;

    participation.pool = pool.key();
    participation.participant = ctx.accounts.participant.key();
    participation.fpl_team_id = fpl_team_id;
    participation.join_time = Clock::get()?.unix_timestamp;
    participation.score = 0;
    participation.bump = *ctx.bumps.get("participation").unwrap();

    pool.participants += 1;

    Ok(())
}