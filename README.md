# [High] Flash Loan Manipulation of Utilization in Rujira Ghost Vault

## 📌 Summary
Finding **S-722** from the Rujira audit. This vulnerability allows attackers to borrow massive amounts of capital at near-zero interest rates, effectively stealing yield from legitimate depositors.

## 🔴 Impact
- **Severity**: High
- **Result**: In a tested scenario, an attacker borrowed **500,000 tokens**, causing the interest rate on existing debt to drop from **20% to 0.1%**.
- **Yield Theft**: Direct loss for protocol depositors as their earned interest is bypassed.
- **Scalability**: The attack can be repeated multiple times to cause permanent loss to the protocol.

## ⚔️ Attack Workflow
1. **Deposit**: Attacker injects a large flash loan into the vault.
2. **Borrow**: Attacker opens/updates positions at the manipulated rate.
3. **Withdraw**: Attacker removes the flash loan liquidity.
The attack is **low cost** and requires only 3 transactions within a single block.

## 🛠 Root Cause
The `rujira-ghost-vault` contract calculates interest rates based on **instantaneous utilization** at the time of each transaction, rather than using a snapshot from the last interest distribution.

## 🛡 Recommendation
Use snapshots from the previous block or implement a Time-Weighted Average (TWAP) for utilization to prevent intra-block manipulation.
