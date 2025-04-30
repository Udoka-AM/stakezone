# Core Platform Components (first round developments

#### Team Import System

- API integration with the official Fantasy Premier League to allow users to import their existing teams securely
- An authentication mechanism to verify ownership of imported teams
- Real-time synchronisation to keep team data current with FPL

#### Prediction Market Infrastructure

- Smart contracts to create and manage prediction pools
- Oracle integration for real-world data feeds of player/team performance
- Automated scoring system that calculates results based on actual Premier League matches

#### Financial System

- Solana wallet integration for deposits and withdrawals
- SPL token support for platform-specific tokens and stablecoins
- Fee structure that's competitive but sustainable

## Key Features
#### Weekly Prediction Markets

- Users stake crypto on their team's weekly performance
- Multiple market types: head-to-head, tournament style, season-long leagues
- Time-locked predictions to prevent last-minute changes

#### Reward System

- Tiered payout structure based on performance percentiles
- Bonus multipliers for consistent performance
- Special rewards for exceptional predictions or season achievements

#### Social Elements
- Leaderboards displaying top performers

### Technical Considerations

#### Development Path

- Build a functional prototype with core features
- Implement the FPL import API
- Create the smart contracts for prediction markets
- Develop the scoring and reward distribution system
- Add social features and user experience enhancements

#### Potential Challenges

- Ensuring accurate and timely data feeds for scoring
- Managing liquidity in prediction pools
- Regulatory compliance, depending on your target markets


## Liquidity Management

## User-Funded Liquidity Model

#### Pool-Based System

- Each prediction market becomes its own liquidity pool
- Entry fees from all participants create the total prize pool
- Smart contracts automatically distribute rewards based on results
- No need for external market makers or liquidity providers

#### Balance Management

- Smart contracts lock funds when predictions are placed
- Funds remain in escrow until scoring and settlement
- Automated distribution of rewards immediately after official results

#### Advantages

- A self-sustaining ecosystem where users provide all necessary liquidity
- No capital requirements for you beyond operating the platform
- Transparent fund management is visible on-chain
- Minimises counterparty risk since funds are held in smart contracts

#### Platform Revenue Model

- Small percentage fee from each pool (e.g., 3-5%)
- Optional premium features or tournaments
- No need to provide house funds for payouts

This approach is particularly well-suited for Solana, as the low transaction fees allow for efficient fund management without eating into users' stakes or winnings. The smart contracts can be programmed to automatically distribute rewards to winners while reserving the platform fee, making the entire process trustless and efficient.


## Winners/losers selection

To determine winners and losers based on real-time FPL team performance, I recommend implementing a percentile-based scoring system with tiered rewards. 
Here's how you could structure it:

### Percentile-Based Scoring System
#### Data Collection

- Pull official FPL scoring data through their API
- Track each player's performance metrics in real-time during matches
- Map imported FPL teams to their corresponding real-world performance

#### Scoring Algorithm

- Calculate the total points for each user's FPL team based on actual player performances
- Rank all participants in the pool from highest to lowest score
- Convert rankings to percentiles (top 1%, top 10%, etc.)
- Assign rewards based on percentile thresholds

#### Reward Distribution Structure

- Top 1%: 20-25% of the pool
- Top 5%: 15-20% of the pool
- Top 10%: 10-15% of the pool
- Top 25%: 5-10% of the pool


This creates a natural curve where exceptional performance is well-rewarded while still providing incentives across multiple tiers.

#### Implementation Benefits

1. Scales automatically with any pool size
2. Creates meaningful distinctions between performance levels
3. Always produces winners regardless of absolute scores
4. Maintains engagement from a broader user base

#### Smart Contract Logic

- Pre-define the percentile thresholds and corresponding reward percentages
- Automatically calculate rankings after all matches complete
- Execute payouts immediately based on final standings

This model is particularly effective because it:

- Rewards skill rather than luck
- Creates multiple winning tiers to keep more users engaged
- Adapts naturally to different pool sizes
- Maintains excitement throughout the competition period


## HOW THIS WORKS


#### Prediction Pool Mechanics in Practice
Let me walk you through how the prediction pools would operate in real-world scenarios:

### Weekly Pool Lifecycle

#### 1. Pool Creation (Pre-Gameweek)

- Administrator or smart contract creates a new pool for the upcoming gameweek
- Entry requirements are set: stake amount, minimum/maximum participants
- Reward tiers are defined (e.g., top 1%, 5%, 10%, 25%, 50%)
- Timeline is established with clear deadlines aligned with FPL deadlines

2. User Participation (Pre-Deadline)

- Users connect their Solana wallets and verify their FPL team ID
- They join the pool by staking the required amount of tokens
- Their current FPL team lineup is registered and locked into the smart contract
- Users receive a token receipt representing their pool entry

3. Pool Lockdown (At Deadline)

- Pool automatically locks when the FPL deadline hits (typically 90 minutes before the first match)
- No further entries or team changes are permitted
- Smart contract displays total pool size and potential rewards

4. Live Scoring (During Matches)

- Oracle pulls live FPL data during matches
- Participants can watch their position change in real-time
- Leaderboard updates show current standings and projected rewards
- No withdrawals are permitted during this phase

5. Settlement (Post-Gameweek)

- Once all matches are complete and FPL points finalised, the oracle triggers settlement
- Smart contract calculates final rankings and percentiles
- Rewards are automatically distributed to winners' wallets based on a tiered structure
- Results are permanently recorded on-chain

### Pool Variations

#### Standard Pools

Open to all users
Fixed entry fee (e.g., 0.1 SOL or 10 USDC)
Predetermined reward structure

#### Premium Pools

Higher stakes (e.g., 1 SOL or 100 USDC)
Steeper reward curve (higher percentage for top performers)
Optional features like insurance against injured players

#### Private Pools

Created by users for friends/communities
Customizable entry fees and reward structures
Invitation-only participation

#### Tournament Pools

Multi-week competition with progressive elimination
A portion of the weekly stakes rolls over to the  final prize pool
Bonus points for consistency across gameweeks


#### Real-World Example
Let's walk through a specific example of how a standard pool would operate:

#### Pool Creation:

Gameweek 12 pool is created on Monday
- Entry fee: 10 USDC
- Maximum participants: 1,000
- Reward structure:

Top 1% (10 users): 25% of pool (25 USDC each)
Top 5% (50 users): 25% of pool (5 USDC each)
Top 10% (100 users): 20% of pool (2 USDC each)
Top 25% (250 users): 20% of pool (0.8 USDC each)
Platform fee: 10% of the pool




## Participation:

By the Friday deadline, 800 users have joined
- Total pool: 8,000 USDC
- Platform fee: 800 USDC
- Distributable prize pool: 7,200 USDC


#### Gameplay:

- Premier League matches occur Saturday through Monday
- Real-time oracle updates scores as matches progress
- Users can track their position on the mobile app/website


#### Settlement:

Final standings calculated Tuesday morning

Top 1% (8 users): 25% of 7,200 USDC = 1,800 USDC (225 USDC each)
Next 4% (32 users): 25% of 7,200 USDC = 1,800 USDC (56.25 USDC each)
Next 5% (40 users): 20% of 7,200 USDC = 1,440 USDC (36 USDC each)
Next 15% (120 users): 20% of 7,200 USDC = 1,440 USDC (12 USDC each)
Next 25% (200 users): 10% of 7,200 USDC = 720 USDC (3.6 USDC each)
Bottom 50% (400 users): No rewards


#### Post-Settlement:

- Winners receive automatic payments to their wallets
- Performance stats added to user profiles
- Invitations sent for next gameweek's pools



#### Technical Execution
Behind the scenes, each step is executed through specific smart contract interactions:

#### Pool Creation Transaction:

- Creates pool account with parameters
- Initializes token escrow account
- Sets oracle connection for data feeds


#### User Join Transaction:

- Verifies user's FPL team ownership
- Transfers stake to escrow
- Creates participant entry in pool


#### Oracle Update Transactions:

- Secured API calls retrieve official FPL data
- Multiple validators confirm data accuracy
- Authenticated oracle updates on-chain scores


#### Settlement Transaction:

- Calculates final rankings with percentile algorithm
- Applies reward distribution formula
- Generates transfer instructions for payouts




## Smart Contract Architecture
#### 1. Main Program Components

```
rust// Key program structures

pub struct PredictionPool {
    pub pool_id: Pubkey,
    pub total_stake: u64,
    pub participants: Vec<Pubkey>,
    pub gameweek: u16,
    pub status: PoolStatus,
    pub fee_percentage: u8,
    pub reward_tiers: Vec<RewardTier>,
}

pub struct Participant {
    pub user: Pubkey,
    pub fpl_team_id: u32,
    pub stake_amount: u64,
    pub current_score: u32,
    pub percentile: Option<u8>,
    pub reward_amount: Option<u64>,
}

pub struct RewardTier {
    pub percentile_threshold: u8,  // e.g., 1 = top 1%
    pub reward_percentage: u8,     // e.g., 25 = 25% of pool
}

pub enum PoolStatus {
    Accepting,      // Pool is open for entries
    Locked,         // Entries locked, awaiting results
    Calculating,    // Processing results
    Completed,      // Results finalized, payouts available
}
```

#### 2. Key Instructions

```
rust// Main program instructions

pub enum PredictionInstruction {
    // Create a new prediction pool
    CreatePool {
        gameweek: u16,
        entry_fee: u64,
        fee_percentage: u8,
        reward_tiers: Vec<RewardTier>,
    },
    
    // Join a prediction pool with your FPL team
    JoinPool {
        pool_id: Pubkey,
        fpl_team_id: u32,
    },
    
    // Lock the pool before gameweek starts
    LockPool {
        pool_id: Pubkey,
    },
    
    // Oracle updates scores during/after matches
    UpdateScores {
        pool_id: Pubkey,
        score_data: Vec<(u32, u32)>,  // (fpl_team_id, score)
    },
    
    // Finalize results and calculate payouts
    FinalizeResults {
        pool_id: Pubkey,
    },
    
    // Claim rewards after pool is finalized
    ClaimReward {
        pool_id: Pubkey,
    },
}
```

#### 3. Oracle Integration
You'll need a reliable oracle system to fetch and verify FPL data:


```
rust// Oracle account structure

pub struct FplOracle {
    pub authority: Pubkey,
    pub last_update: i64,
    pub player_scores: HashMap<u32, u32>,  // player_id -> score
}

// Oracle update instruction
pub fn process_oracle_update(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    player_scores: HashMap<u32, u32>,
) -> ProgramResult {
    // Verify oracle authority
    // Update player scores
    // Emit events for transparency
}
```

#### 4. Scoring Algorithm Implementation
Here's the core algorithm for calculating percentiles and rewards:


```
rust

pub fn calculate_results(
    pool: &mut PredictionPool,
    participants: &mut [Participant],
) -> ProgramResult {
    // 1. Ensure all scores are updated
    if participants.iter().any(|p| p.current_score == 0) {
        return Err(ProgramError::IncompleteScoring);
    }
    
    // 2. Sort participants by score (descending)
    participants.sort_by(|a, b| b.current_score.cmp(&a.current_score));
    
    // 3. Calculate percentiles
    let total_participants = participants.len();
    for (index, participant) in participants.iter_mut().enumerate() {
        // Calculate percentile position (0 = top performer)
        let percentile = ((index as f64) / (total_participants as f64) * 100.0) as u8;
        participant.percentile = Some(percentile);
    }
    
    // 4. Calculate rewards based on percentiles
    let mut remaining_pool = pool.total_stake;
    let platform_fee = (pool.total_stake * pool.fee_percentage as u64) / 100;
    remaining_pool -= platform_fee;
    
    // Sort reward tiers by percentile (ascending)
    let mut sorted_tiers = pool.reward_tiers.clone();
    sorted_tiers.sort_by_key(|tier| tier.percentile_threshold);
    
    // Apply rewards for each tier
    for tier in sorted_tiers {
        let tier_total = (remaining_pool * tier.reward_percentage as u64) / 100;
        let eligible_count = participants
            .iter()
            .filter(|p| p.percentile.unwrap() <= tier.percentile_threshold)
            .count();
            
        if eligible_count > 0 {
            let reward_per_participant = tier_total / eligible_count as u64;
            
            for participant in participants.iter_mut() {
                if participant.percentile.unwrap() <= tier.percentile_threshold {
                    participant.reward_amount = Some(reward_per_participant);
                }
            }
        }
    }
    
    pool.status = PoolStatus::Completed;
    Ok(())
}
```

#### 5. Token Handling

```
rust// Stake tokens when joining a pool

pub fn process_join_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fpl_team_id: u32,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let user = next_account_info(account_info_iter)?;
    let pool_account = next_account_info(account_info_iter)?;
    let participant_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    
    // Verify pool is accepting entries
    let mut pool = PredictionPool::deserialize(&pool_account.data.borrow())?;
    if pool.status != PoolStatus::Accepting {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Transfer tokens from user to pool
    let transfer_instruction = spl_token::instruction::transfer(
        token_program.key,
        user_token_account.key,
        pool_token_account.key,
        user.key,
        &[],
        pool.entry_fee,
    )?;
    
    invoke(
        &transfer_instruction,
        &[
            user_token_account.clone(),
            pool_token_account.clone(),
            user.clone(),
            token_program.clone(),
        ],
    )?;
    
    // Create participant record
    let participant = Participant {
        user: *user.key,
        fpl_team_id,
        stake_amount: pool.entry_fee,
        current_score: 0,
        percentile: None,
        reward_amount: None,
    };
    
    // Update pool data
    pool.participants.push(*user.key);
    pool.total_stake += pool.entry_fee;
    
    // Save updated data
    pool.serialize(&mut pool_account.data.borrow_mut())?;
    participant.serialize(&mut participant_account.data.borrow_mut())?;
    
    Ok(())
}
```

#### Program Architecture Benefits

- Composability: Works with SPL tokens and other Solana programs
- Scalability: Efficient data structures minimize transaction costs
- Transparency: All calculations and results are on-chain and verifiable
- Flexibility: Reward tiers can be adjusted for different pools
- Security: Oracle data updates are verified and access-controlled

#### Additional Technical Considerations

#### Oracle Reliability

- Implement multiple data sources for FPL scores to ensure accuracy
- Use a time-weighted average to smooth any inconsistencies
- Create contingency mechanisms for oracle failures


#### Gas Optimization

- Batch score updates to minimize transaction costs
- Use off-chain computation where possible, with on-chain verification


#### Upgradability

- Consider implementing an upgradeable contract pattern
- Store configuration in a separate account for easier updates


#### Cross-Program Invocation

- Interface with other DeFi protocols for additional features
- Enable token swaps for users who want to use different currencies


