# TRO Governance Staking

A MultiversX smart contract for TRO token governance through staking. The contract allows users to stake both TRO tokens and TRO LP tokens to participate in governance decisions.

## Overview

The smart contract implements a governance system where users can stake their TRO tokens (either directly or through LP tokens) to participate in voting on proposals. The voting power is calculated based on the total TRO equivalent at the time of proposal creation.

### Key Features

- Stake TRO tokens
- Stake TRO LP tokens from supported pairs
- Participate in governance voting
- View voting power and stake information
- Unstake tokens (locked during active proposals)

## Technical Details

### Staking
- Support for direct TRO token staking
- Support for TRO LP token staking
- Automatic calculation of voting power
- Unstaking lock during active proposals

### Governance
- Owner-created proposals
- Voting power based on total TRO equivalent
- Support for multiple LP pair types
- Real-time voting power calculation

### Analytics
- Comprehensive view functions
- Stake information
- Voting power calculation
- Proposal status tracking