use anchor_lang::prelude::*;
use crate::instructions::*;
use crate::state::*;
use crate::errors::*;

declare_id!("5m6vknYePNRhEecTTnkW7L9NhZxrgK9A2axnVpyDPkFF"); // Replace with your actual program ID

pub mod instructions {
    pub mod pool_create;
    pub mod pool_join;
    pub mod results_record;
    pub mod rewards_distribution;
}

pub mod state {
    pub mod pool;
    pub mod participation;
}

pub mod errors;

#[program]
mod fantasy_sports {
    use super::*;

    pub fn create_pool(ctx: Context<instructions::create_pool::CreatePool>, entry_fee: u64, reward_tiers: Vec<(u8, u8)>, start_time: i64, end_time: i64) -> Result<()> {
        instructions::create_pool::handler(ctx, entry_fee, reward_tiers, start_time, end_time)
    }

    pub fn join_pool(ctx: Context<instructions::join_pool::JoinPool>, fpl_team_id: u32) -> Result<()> {
        instructions::join_pool::handler(ctx, fpl_team_id)
    }

    pub fn record_results(ctx: Context<instructions::record_results::RecordScore>, score: u16) -> Result<()> {
        instructions::record_results::handler(ctx, score)
    }

    pub fn distribute_rewards(ctx: Context<instructions::distribute_rewards::DistributeRewards>) -> Result<()> {
        instructions::distribute_rewards::handler(ctx)
    }
}