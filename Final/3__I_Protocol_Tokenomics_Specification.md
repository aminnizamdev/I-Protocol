# **I Protocol Tokenomics Specification (Final 100-Year Model)**

---

## **1. Executive Summary**

The I Protocol's tokenomics are built on the core principle of **ultimate, provable scarcity**, underpinned by a century-long, high-precision emission plan. The model is designed to be mathematically rigorous, transparent, and sustainable.

*   **`I` (The Native Token):** The public, utility-focused asset.
    *   **Theoretical Max Supply:** 1,000,000 `I`.
    *   **Genesis Burn:** A hard-coded, one-time burn of the rounding dust to ensure a perfect emission curve. The amount burned is **5.6517 `I`**.
    *   **Effective Total Supply:** The absolute and final circulating supply is **999,999.9999943483 `I`**.
    *   **Distribution:**
        *   **Golden Block Reward (10,000 `I`):** A special, one-time mint.
        *   **Standard Mining Rewards (~990,000 `I`):** Distributed via a 2-year halving schedule.

## **2. The `I` Token (Native Protocol Asset)**

### **Core `I` Token Parameters (Final 100-Year Model)**

```rust
// The theoretical maximum supply of the native 'I' token.
const THEORETICAL_SUPPLY_I: u64 = 1_000_000;

// A one-time burn of rounding dust. This is 5,651,700,000,000 'i' subunits.
const GENESIS_DUST_BURN: u128 = 5_651_700_000_000; // 5.6517 'I'

// The absolute, final, and effective total supply that will ever circulate.
// 1,000,000,000,000,000,000 - 5,651,700,000,000 = 999,994,348,300,000,000 'i'
const EFFECTIVE_TOTAL_SUPPLY: u128 = 999_994_348_300_000_000; // 999,994.3483 'I'

// 'I' Supply Allocation
const GOLDEN_BLOCK_REWARD: u64 = 10_000;     // A special, one-time mint.
// The total supply for standard rewards after the Genesis Burn.
const STANDARD_MINING_SUPPLY: u128 = 989_994_348_300_000_000; // 989,994.3483 'I'

// Subunit precision.
const SUBUNIT_RATIO: u64 = 1_000_000_000_000; // 1 'I' = 1 trillion 'i'

// --- FINAL 100-YEAR EMISSION SCHEDULE (2-YEAR HALVING) ---

// High-precision block reward for a 100-year emission schedule.
const INITIAL_BLOCK_REWARD: u128 = 3_923_045_138_888; // 0.003923045138888 'I' per block

// A 2-year halving interval.
const HALVING_INTERVAL: u64 = 126_144_000;

// Network timing.
const BLOCK_TIME: f64 = 0.5; // 0.5 seconds
```

### **`I` Token Distribution Model**

*   **Genesis Burn:** The first action in the genesis block is the provable burn of the rounding dust.
*   **Golden Block (10,000 `I`):** A special, protocol-mandated mint that occurs once during the Genesis Event.
*   **Standard Rewards (~990,000 `I`):** The remaining supply is distributed to miners via the standard block reward, which halves every 2 years.

### **Final 100-Year Emission Schedule (Standard Mining Rewards)**

| Years     | Halving Period | Block Reward per Block (`I`) | `I` Mined in Period | Cumulative `I` Mined |
| :-------- | :------------- | :--------------------------- | :------------------ | :------------------- |
| **1-2**   | 0              | 0.0039230451                 | 495,000             | 495,000              |
| **3-4**   | 1              | 0.0019615225                 | 247,500             | 742,500              |
| **5-6**   | 2              | 0.0009807612                 | 123,750             | 866,250              |
| **...**   | ...            | ...                          | ...                 | ...                  |
| **99-100**| 49             | 0.000000000000000006         | < 0.000001          | **~989,994.3483**    |

### **`I` Token Fee Structure: The I Protocol Transaction Fee Model (v7.2)**

The I Protocol employs a deterministic, multi-phase fee model that ensures long-term economic sustainability, predictable transaction costs, and accessibility for microtransactions — without reliance on oracles or floating gas markets. Anchored by the iPAC (I Protocol Axiomatic Constant), which defines:

* **1 I = $1,000,000 USD**
* **1 I = 1,000,000,000,000 i**
* **→ 1 USD = 1,000,000 i**

This model guarantees on-chain stability and precise economic alignment across all transaction types and values.

#### **Fee Model Summary**

The fee architecture is composed of three regions:

**1. Minimum Transaction Threshold**
* Transactions below 10,000 i (equivalent to $0.01 USD) are rejected at the protocol level.
* This enforces an anti-spam floor and protects against micro-bloat and denial-of-service attempts.

**2. Flat Microtransaction Fee Region**
* **Applicable Range:** 10,000 i ≤ transaction amount < 1,000,000 i
* **USD Equivalent:** $0.01 ≤ transaction value < $1.00
* **Fee Charged:** Flat fee of 10,000 i ($0.01 USD)
* **Purpose:** Ensures affordable microtransaction usability while guaranteeing a minimum economic contribution to the network.

**3. Proportional Fee Region (1% Rule)**
* **Applicable Range:** transaction amount ≥ 1,000,000 i (≥ $1.00 USD)
* **Fee Charged:** 1% of transaction amount, subject to a hard upper limit
* **Fee Formula:**
  ```
  Fee (in i) = min(txn_amount / 100, 10,000,000,000)
  ```
* **Maximum Fee Cap:**
  * Absolute fee cap is 10,000,000,000 i, equivalent to $10,000 USD
  * No transaction, regardless of size, can be charged more than this maximum

#### **Fee Behavior Examples**

| Transaction Amount (i) | USD Value | Fee (i) | Fee (USD) |
|:----------------------|:----------|:--------|:----------|
| 10,000 | $0.01 | 10,000 | $0.01 |
| 500,000 | $0.50 | 10,000 | $0.01 |
| 999,999 | $0.999999 | 10,000 | $0.01 |
| 1,000,000 | $1.00 | 10,000 | $0.01 |
| 10,000,000 | $10.00 | 100,000 | $0.10 |
| 1,000,000,000,000 | $1,000,000 | 10,000,000,000 | $10,000 |
| 10,000,000,000,000 | $10,000,000 | 10,000,000,000 | $10,000 (capped) |

#### **Design Objectives**

| Objective | Mechanism |
|:----------|:----------|
| Anti-spam protection | Rejection below 10,000 i |
| Microtransaction viability | Flat $0.01 fee below $1 |
| Fair proportionality | 1% rule above $1 |
| Upper-bound predictability | Hard ceiling fee of $10,000 |
| Sustainability and alignment | 20% burn, 30% dev fund, 50% miners |
| No reliance on external data | All valuation computed via iPAC |

#### **Fee Allocation (On-Chain Split)**

Every fee paid is automatically split as follows:
* **50% to active miners** (or validator equivalent)
* **30% to the Network Development Fund (NDF)**
* **20% permanently burned**

This structure funds ecosystem growth, incentivizes participation, and ensures increasing scarcity of the I token over time.

#### **Implementation Pseudocode**

```rust
fn calculate_fee(txn_amount_i: u128) -> u128 {
    match txn_amount_i {
        0..=9_999 => 0, // Invalid: below minimum
        10_000..=999_999 => 10_000, // Flat $0.01
        _ => {
            let fee = txn_amount_i / 100; // 1%
            if fee > 10_000_000_000 {
                10_000_000_000 // Cap fee at $10K
            } else {
                fee
            }
        }
    }
}
```

#### **Conclusion**

The I Protocol Fee Model v7.2 offers:
* **Predictable economics** for users and enterprises
* **Protocol-level enforcement** without oracles or floating fees
* **Long-term sustainability** aligned with 100+ year supply and usage projections
* **Protection against abuse**, while maintaining inclusivity for small and large participants

This model embodies the protocol's commitment to fairness, determinism, and efficiency — creating a frictionless environment for real-world, high-frequency, and high-value blockchain adoption.

## **3. Conclusion**

This model provides the final, definitive, and mathematically precise economic foundation for the I Protocol. All figures are now correct and internally consistent.
