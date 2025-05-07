use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Mint};
use crate::state::pool::Pool;
use crate::state::participation::Participation;
use std::cmp::Reverse;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.creator.as_ref()],
        bump = pool.bump,
        has_one = usdc_mint,
        has_one = fee_payer,
        constraint = pool.end_time < Clock::get()?.unix_timestamp && pool.is_active, // Ensure pool ended and is active
    )]
    pub pool: Account<'info, Pool>,
    pub usdc_mint: Account<'info, Mint>, // Ensure the pool is using the correct USDC mint
    #[account(mut, seeds = [b"pool_usdc", pool.key().as_ref()], bump)]
    pub pool_usdc_account: Account<'info, TokenAccount>, // Pool's USDC escrow account
    /// CHECK: Authority to sign for the pool's USDC account.
    #[account(signer)]
    pub reward_authority: AccountInfo<'info>,
    #[account(mut)]
    pub fee_payer: SystemAccount<'info>, // Account to receive platform fees (in SOL)
    /// CHECK: Remaining accounts are the winners' USDC token accounts.
    pub winner_usdc_accounts: RemainingAccounts<'info>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> DistributeRewards<'info> {
    fn transfer_reward(&self, winner_usdc_account: &AccountInfo<'info>, amount: u64) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pool_usdc_account.to_account_info(),
            to: winner_usdc_account.to_account_info(),
            authority: self.reward_authority.to_account_info(), // The program's or pool's authority signs
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<DistributeRewards>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let pool_usdc_account = &ctx.accounts.pool_usdc_account;
    let fee_payer = &ctx.accounts.fee_payer;
    let token_program = &ctx.accounts.token_program;
    let reward_authority = &ctx.accounts.reward_authority;
    let usdc_mint = &ctx.accounts.usdc_mint;

    // Calculate the total prize pool (in USDC)
    let total_prize_pool = pool_usdc_account.amount;
    let platform_fee_usdc = total_prize_pool * 10 / 100; // Example: 10% platform fee in USDC
    let distributable_prize_pool = total_prize_pool - platform_fee_usdc;

    // Fetch all Participation accounts for this pool
    let participants = Participation::fetch_multiple(
        &ctx.program_id,
        &[("pool", pool.key().as_ref())],
    )?;

    // Sort participants by score in descending order
    let mut sorted_participants = participants;
    sorted_participants.sort_by_key(|p| Reverse(p.score));

    let num_participants = sorted_participants.len() as u64;
    let mut winners_count = 0;

    for (rank_percentage, reward_percentage) in pool.reward_tiers.iter() {
        let threshold = (*rank_percentage as f64 / 100.0 * num_participants as f64).ceil() as u64;
        let num_winners_in_tier = (winners_count..threshold).len() as u64;

        if num_winners_in_tier > 0 {
            let reward_amount_per_winner = (distributable_prize_pool * (*reward_percentage as u64)) / 100 / num_winners_in_tier;

            for i in winners_count..threshold {
                if let Some(participant) = sorted_participants.get(i as usize) {
                    // Find the winner's USDC token account in the remaining accounts
                    let winner_usdc_account_info = ctx.remaining_accounts.iter().find(|account| {
                        account.owner == token_program.key() &&
                        account.data_is_empty() == false &&
                        account.try_borrow_data().map_or(false, |data| {
                            let account_data = token::TokenAccount::try_deserialize(&mut &data[..]).unwrap();
                            account_data.owner == participant.participant && account_data.mint == usdc_mint.key()
                        })
                    });

                    if let Some(winner_account) = winner_usdc_account_info {
                        token::transfer(
                            ctx.accounts.transfer_reward(&winner_account, reward_amount_per_winner)?,
                            reward_amount_per_winner,
                        )?;
                        winners_count += 1;
                    } else {
                        msg!("Warning: Winner USDC token account not found for participant {}", participant.participant);
                    }
                }
            }
        }
    }

    // Transfer platform fees (in SOL) to the fee payer
    if platform_fee_usdc > 0 {
        let ix = anchor_lang::system_program::transfer(
            ctx.accounts.reward_authority.key(), // Assuming reward authority pays SOL fees
            ctx.accounts.fee_payer.key(),
            platform_fee_usdc / 100_000_000, // Example: Convert a fraction of USDC to SOL for fees (very rough)
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.reward_authority.to_account_info(),
                ctx.accounts.fee_payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"pool", pool.creator.as_ref(), &[pool.bump]]],
        )?;
    }

    pool.is_active = false; // Mark the pool as settled

    Ok(())
}