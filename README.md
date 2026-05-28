# No-Loss Auction Protocol

A decentralized no-loss auction protocol built on the Stellar blockchain using Soroban smart contracts.

## Project Overview

This project allows users to participate in auctions using SEP-41 tokens without permanently losing funds when they are outbid.

Unlike traditional auction systems where non-winning bidders may lose access to their funds temporarily or permanently, this protocol ensures that outbid users can reclaim their tokens through a secure refund mechanism.

The protocol is powered by Soroban smart contracts on Stellar and includes a frontend interface for interacting with auction functionalities.

---

# Core Features

## Create Auction
Users can create auctions by specifying:
- Auction item
- Starting bid
- Auction deadline

## Place Bids Using SEP-41 Tokens
Participants place bids using SEP-41 compatible tokens on Stellar.

## Highest Bid Tracking
The contract continuously tracks:
- Current highest bid
- Current highest bidder

## No-Loss Refund Mechanism
When a participant is outbid:
- Their funds are not permanently lost
- Refund balances are recorded
- Users can reclaim their tokens safely

## Claim Refund
Outbid bidders can claim their refundable token balances directly from the contract.

## Finalize Auction
After the auction deadline:
- The auction is finalized
- The highest bidder wins
- The seller receives the winning amount

## Cancel Auction
Auction creators can cancel auctions only if:
- No bids have been placed

---

# Smart Contract Architecture

The project uses:
- Soroban Smart Contracts
- Rust Programming Language
- SEP-41 Token Standard
- Stellar Testnet

---

# Frontend Features

The frontend integrates all contract functionalities including:
- Auction creation
- Bid placement
- Refund claiming
- Auction finalization
- Auction listing and viewing

---

# Project Structure

```text
No_Loss_Auction_Protocol/
│
├── contracts/
│   └── no-loss-auction/
│
├── frontend/
│
├── README.md
│
└── .gitignore
```

---

# Technologies Used

- Rust
- Soroban SDK
- Stellar Blockchain
- SEP-41 Token Standard
- React / Next.js
- Freighter Wallet

---

# Deployment

## Smart Contract
The contract will be deployed to Stellar Testnet.

Contract ID:
```text
To be added after deployment
```

## Frontend
Frontend deployment link:
```text
To be added after deployment
```

---

# Testing

The project includes unit tests for:
- Auction creation
- Bid placement
- Refund claims
- Auction finalization
- Auction cancellation

---

# No-Loss Auction Logic

The protocol ensures that only the final winning bidder pays permanently.

When users are outbid:
- Their funds become refundable
- They can claim their tokens back from the contract

This creates a safer and fairer decentralized auction system.

---

# Author

Charlo Juma
