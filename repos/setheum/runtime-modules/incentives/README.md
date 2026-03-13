# Incentives Module

Provides Incentive mechanisms for Ethical DeFi Protocols.

## Overview

 Exchange needs to support multiple open liquidity reward mechanisms. Each Pool has  its own multi currencies rewards and reward accumulation mechanism. module rewards records the total shares, total multi currencies rewards and user shares of specific pool.  Incentives module provides hooks to other protocals to manage shares, accumulates rewards and distributes rewards to users based on their shares.

## Pool types:

2. UssdLiquidityRewards: record the shares and rewards for Setheum USD (SEUSD) users who are staking LP tokens.
3. LiquidityRewards: record the shares and rewards for  makers who are staking LP token.

## Rewards accumulation:

Rewards: periodicly(AccumulatePeriod), accumulate fixed amount according to Rewards. Rewards come from RewardsSource, please transfer enough tokens to RewardsSource before start Rewards plan.
