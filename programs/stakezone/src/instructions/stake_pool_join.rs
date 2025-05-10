use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Mint, Token};
use crate::state::pool::Pool;
use crate::state::participation::Participation;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(fpl_team_id: u32)]
pub struct JoinStakePool<'info> {
    #[account(mut, signer)]
    pub participant: Signer<'info>,
    #[account(
        mut,
        seeds = [b"pool", pool.creator.as_ref()],
        bump = pool.bump,
        has_one = usdc_mint,
        constraint = pool.is_active && Clock::get()?.unix_timestamp >= pool.start_time && Clock::get()?.unix_timestamp < pool.end_time,
    )]
    pub pool: Account<'info, Pool>,
    pub usdc_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = participant,
        space = 8 + Participation::MAX_SIZE,
        seeds = [b"participation", pool.key().as_ref(), participant.key().as_ref()],
        bump,
    )]
    pub participation: Account<'info, Participation>,
    #[account(
        mut,
        constraint = participant_usdc_account.mint == usdc_mint.key(),
        constraint = participant_usdc_account.owner == participant.key()
    )]
    pub participant_usdc_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = reward_authority,
    )]
    pub pool_usdc_account: Account<'info, TokenAccount>,
    /// CHECK: Authority to sign for the pool's USDC account
    #[account(signer)]
    pub reward_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinStakePool<'info> {
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
    ctx: Context<JoinStakePool>,
    fpl_team_id: u32,
) -> Result<()> {
    // Check if participant has enough USDC
    require!(
        ctx.accounts.participant_usdc_account.amount >= ctx.accounts.pool.entry_fee,
        ErrorCode::InsufficientFunds
    );

    // Transfer entry fee
    token::transfer(ctx.accounts.transfer_entry_fee(), ctx.accounts.pool.entry_fee)?;

    let pool = &mut ctx.accounts.pool;
    let participation = &mut ctx.accounts.participation;

    // Initialize participation
    participation.pool = pool.key();
    participation.participant = ctx.accounts.participant.key();
    participation.fpl_team_id = fpl_team_id;
    participation.join_time = Clock::get()?.unix_timestamp;
    participation.score = 0;
    participation.bump = ctx.bumps.participation;

    // Update pool
    pool.participants += 1;

    Ok(())
}