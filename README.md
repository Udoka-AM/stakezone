# Core Platform Components 
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
