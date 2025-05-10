use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RewardTier {
    pub threshold: u8,
    pub percentage: u8,
} 