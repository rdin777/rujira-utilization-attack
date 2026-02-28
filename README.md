# [High] Flash Loan Utilization Manipulation in Rujira Ghost Vault

## 📌 Summary
Finding **S-722** from the Rujira Ghost Vault audit. This vulnerability allows an attacker to manipulate the interest rate model using Flash Loans, enabling borrowing at near-zero rates.

## 🔴 The Vulnerability
The contract `rujira-ghost-vault` calculates the interest rate based on **instantaneous utilization**:
- **Utilization Rate** = Total Debt / Total Liquidity.
- The interest rate is a function of this utilization.

By injecting a massive amount of liquidity via a Flash Loan, the attacker artificially increases the "Total Liquidity," which causes the **Utilization Rate** to drop to near-zero. Consequently, the interest rate for any new debt positions opened in the same block also drops to zero.

## ⚔️ Attack Scenario
1. **Flash Loan**: Attacker takes a large Flash Loan of the underlying asset.
2. **Deposit**: Attacker deposits the Flash Loan into the Vault.
3. **Borrow**: Attacker opens a large, long-term debt position. Due to the manipulated utilization, the interest rate is ~0%.
4. **Withdraw**: Attacker withdraws their initial deposit.
5. **Repay**: Attacker repays the Flash Loan.
6. **Result**: The attacker now holds a massive debt position that accrues no (or minimal) interest.

## 🛡️ Recommendation
- Use **utilization snapshots** from the previous block.
- Implement a **Time-Weighted Average Utilization (TWAU)** to prevent intra-block manipulation.
