# Project Title

Stellar Split Payment – A Soroban Smart Contract dApp for Group Expense Splitting on Stellar

## Project Vision

This project demonstrates **group expense splitting on the Stellar blockchain** using Soroban smart contracts. It provides a transparent, trustless mechanism for:
- Creating payment splits with multiple recipients
- Collecting contributions for a split
- Automatically distributing funds proportionally to recipients

The goal is to provide a simple, working example of multi-party financial arrangements on Stellar.

---

## Description

A Soroban smart contract dApp that enables **group expense splitting** on Stellar Testnet. An admin creates a payment split with recipients and their respective share percentages (must sum to 100), anyone can contribute funds, and the admin triggers distribution to send funds proportionally to all recipients.

---

## Features

### 1. Admin-Managed Splits
- Admin creates splits with unique IDs
- Recipients and share percentages defined at creation
- Shares must sum to 100 (percentages)

### 2. Open Contributions
- Anyone can contribute XLM to an active split
- Contract receives and holds funds until distribution

### 3. Proportional Distribution
- Admin triggers distribution after contributions
- Funds distributed automatically based on share percentages
- Transparent, on-chain settlement

### 4. Query Interface
- Anyone can query split details (recipients, shares, total received)
- View distribution status

---

## Contract Functions

- **create_split(split_id, recipients, shares)** – Admin creates a new payment split
- **contribute(split_id)** – Anyone sends funds to a split
- **distribute(split_id)** – Admin triggers proportional distribution
- **get_split(split_id)** – Returns split details

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CAFFNSK5ILRQV2T7HLX3WSICFVPL4OA75CVFBQ2BFTMRRGOSRZMLC342](https://stellar.expert/explorer/testnet/tx/4da403dae803b988610e95213de4f1ce5b8694a0003742ac4ffa008aed0e4455)

![screenshot](https://i.ibb.co/9HYHcVFc/image.png)

---

## Future Scopes

### 1. Time-Locked Distribution
- Automatic distribution after a certain time period

### 2. Escrow Protection
- Add a reveal-then-claim mechanism for additional security

### 3. Frontend dApp
- Build a web interface using React for easier split creation and tracking

### 4. Multiple Currencies
- Support for custom Stellar tokens (Soroban Token) in addition to native XLM

### 5. Partial Distribution
- Allow distribution before 100% of expected funds are received

### 6. Split Modification
- Allow admin to add/remove recipients before distribution

### 7. Governance
- Let recipients vote on whether to accept or reject a split
---

## Profile

- **Name:** :williamsonvhaidenv-lang

