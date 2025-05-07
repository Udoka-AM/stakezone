use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Pool is not open for joining.")]
    PoolClosed,
    #[msg("The pool start time has not been reached yet.")]
    PoolNotStarted,
    #[msg("The pool end time has passed.")]
    PoolEnded,
    #[msg("Insufficient funds to join the pool.")]
    InsufficientFunds,
    #[msg("You have already joined this pool.")]
    AlreadyJoined,
    #[msg("Invalid reward tier configuration.")]
    InvalidRewardTier,
    #[msg("Reward tiers must sum to 100%.")]
    InvalidRewardTierSum,
    #[msg("Oracle account provided is not authorized for this pool.")]
    UnauthorizedOracle,
    #[msg("The pool has not ended yet.")]
    PoolNotSettled,
    #[msg("Scores cannot be recorded before the pool end time.")]
    ScoreRecordingTooEarly,
    #[msg("Price feed is unavailable.")]
    PriceFeedUnavailable,
    #[msg("Invalid number of winner accounts provided.")]
    InvalidWinnerAccountCount,
    #[msg("Winner token account does not match the participant.")]
    InvalidWinnerTokenAccount,
    #[msg("The pool has already been settled and rewards distributed.")]
    PoolAlreadySettled,
    #[msg("Entry fee must be greater than zero.")]
    InvalidEntryFee,
    #[msg("Start time must be before end time.")]
    InvalidTimeframe,
}