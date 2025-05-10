use anchor_lang::prelude::*;
use crate::state::RewardTier;

declare_id!("CK7aWW2GLBKg3c495z5Rcptrfgcm7DQYq8rTC5dVgEHV");

pub mod errors;
pub mod state;
pub mod instructions;

use instructions::stake_pool_initialize::*;
use instructions::stake_pool_join::*;

#[program]
pub mod stakezone {
    use crate::state::RewardTier;

    use super::*;

    // Pool Management
    pub fn initialize_stake_pool(
        ctx: Context<InitializeStakePool>,
        entry_fee: u64,
        reward_tiers: Vec<RewardTier>,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        instructions::stake_pool_initialize::handler(ctx, entry_fee, reward_tiers, start_time, end_time)
    }

    pub fn join_stake_pool(
        ctx: Context<JoinStakePool>,
        fpl_team_id: u32,
    ) -> Result<()> {
        msg!("Joining stake pool...");
        instructions::stake_pool_join::handler(ctx, fpl_team_id)
    }
}

