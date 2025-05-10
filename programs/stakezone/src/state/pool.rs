use anchor_lang::prelude::*;
use crate::state::RewardTier;

#[account]
pub struct Pool {
    pub creator: Pubkey,
    pub entry_fee: u64,
    pub reward_tiers: Vec<RewardTier>,
    pub start_time: i64,
    pub end_time: i64,
    pub participants: u32,
    pub is_active: bool,
    pub usdc_mint: Pubkey, // Add the USDC mint address
    pub bump: u8,
}

impl Pool {
    pub const MAX_REWARD_TIERS: usize = 10; // Example maximum number of reward tiers

    pub const MAX_SIZE: usize = 8 + // Discriminator
                                32 + // creator
                                8 + // entry_fee
                                (4 + (Pool::MAX_REWARD_TIERS * (1 + 1))) + // reward_tiers (Vec length + (u8, u8) * max)
                                8 + // start_time
                                8 + // end_time
                                4 + // participants
                                1 + // is_active
                                32 + // usdc_mint
                                1;  // bump
}