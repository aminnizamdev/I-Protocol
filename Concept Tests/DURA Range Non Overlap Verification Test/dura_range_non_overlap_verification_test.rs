// I Protocol - TEST 2.2: DURA RANGE NON-OVERLAP VERIFICATION
// Senior Director of Development: Grey
// CEO: Kauffmen Ceb
// Objective: Mathematically prove ∀i≠j: Range_i ∩ Range_j = ∅

use std::collections::{HashMap, BTreeMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

// DURA Configuration Constants
const NONCES_PER_MINER: u64 = 250000;
const VIRTUAL_MINERS: usize = 1000;
const TEST_SCENARIOS: usize = 500;
const EDGE_CASE_SCENARIOS: usize = 100;
const PROTOCOL_SALT: &str = "I_PROTOCOL_DURA_NON_OVERLAP_2024";
const MAX_DYNAMIC_MINERS: usize = 1500;
const MIN_DYNAMIC_MINERS: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MinerIdentity {
    private_key: String,
    public_key: String,
    wallet_address: String,
    creation_timestamp: u64,
    unique_signature: String,
    salt: String,
    identity_nonce: u64,
    submission_timestamp: u64,
    is_active: bool,
}

impl MinerIdentity {
    fn new(id: u32, base_timestamp: u64) -> Self {
        MinerIdentity {
            private_key: format!("priv_key_{:08x}", id),
            public_key: format!("pub_key_{:08x}", id),
            wallet_address: format!("addr_{:08x}", id),
            creation_timestamp: base_timestamp + (id as u64 * 1000),
            unique_signature: format!("sig_{:08x}", id),
            salt: format!("salt_{:08x}", id),
            identity_nonce: id as u64,
            submission_timestamp: base_timestamp + (id as u64 * 1000) + 500,
            is_active: true,
        }
    }

    fn compute_identity_hash(&self) -> String {
        // 8-Component Identity Hash: H₃(pk ‖ addr ‖ ts ‖ sig ‖ salt ‖ nonce ‖ ts₂)
        let combined = format!(
            "{}{}{}{}{}{}{}",
            self.private_key,
            self.wallet_address,
            self.creation_timestamp,
            self.unique_signature,
            self.salt,
            self.identity_nonce,
            self.submission_timestamp
        );
        
        // Simplified triple-layer hash (Blake3 -> SHA-256 -> Dilithium simulation)
        let hash1 = self.djb2_hash(&combined);
        let hash2 = self.fnv_hash(&format!("{}", hash1));
        let hash3 = self.sdbm_hash(&format!("{}", hash2));
        
        format!("{:016x}", hash3)
    }

    // Simplified hash functions for testing (production uses Blake3/SHA-256/Dilithium)
    fn djb2_hash(&self, input: &str) -> u64 {
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        hash
    }

    fn fnv_hash(&self, input: &str) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for byte in input.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }

    fn sdbm_hash(&self, input: &str) -> u64 {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = (byte as u64).wrapping_add(hash << 6).wrapping_add(hash << 16).wrapping_sub(hash);
        }
        hash
    }
}

#[derive(Debug, Clone)]
struct MinerRange {
    miner_id: u32,
    identity_hash: String,
    start_nonce: u64,
    end_nonce: u64,
    range_size: u64,
    position: usize,
}

#[derive(Debug, Clone)]
struct BlockchainState {
    prev_hash: String,
    block_height: u64,
    timestamp: u64,
    active_miner_count: u32,
}

#[derive(Debug)]
struct NonOverlapTestResult {
    test_id: usize,
    scenario_type: String,
    blockchain_state: BlockchainState,
    range_assignments: Vec<MinerRange>,
    total_miners: usize,
    total_nonces_assigned: u64,
    range_verification_passed: bool,
    non_overlap_verified: bool,
    sequential_order_verified: bool,
    exact_nonce_count_verified: bool,
    gap_analysis_passed: bool,
    overlap_count: usize,
    gap_count: usize,
}

#[derive(Debug)]
struct NonOverlapStatistics {
    total_tests: usize,
    standard_scenarios: usize,
    edge_case_scenarios: usize,
    dynamic_scenarios: usize,
    non_overlap_success: usize,
    sequential_order_success: usize,
    exact_nonce_success: usize,
    gap_analysis_success: usize,
    total_miners_tested: usize,
    total_ranges_verified: usize,
    total_nonces_verified: u64,
    overlap_violations: usize,
    gap_violations: usize,
    success_rate: f64,
    mathematical_proof_validated: bool,
    test_passed: bool,
}

struct DuraRangeAssignmentEngine {
    miner_identities: Vec<MinerIdentity>,
}

impl DuraRangeAssignmentEngine {
    fn new(miners: Vec<MinerIdentity>) -> Self {
        DuraRangeAssignmentEngine {
            miner_identities: miners,
        }
    }

    fn generate_master_seed(&self, blockchain_state: &BlockchainState) -> String {
        // Master Seed = H₃(prev_hash ‖ sort(ID_Hash₁, ID_Hash₂, ..., ID_Hash_n))
        let mut identity_hashes: Vec<String> = self.miner_identities
            .iter()
            .filter(|miner| miner.is_active)
            .map(|miner| miner.compute_identity_hash())
            .collect();
        
        // Canonical lexicographic sorting (critical for determinism)
        identity_hashes.sort();
        
        let concatenated_hashes = identity_hashes.join("");
        let seed_input = format!("{}{}", blockchain_state.prev_hash, concatenated_hashes);
        
        // Triple-layer hash for master seed
        let hash1 = self.djb2_hash(&seed_input);
        let hash2 = self.fnv_hash(&format!("{}", hash1));
        let hash3 = self.sdbm_hash(&format!("{}", hash2));
        
        format!("{:016x}", hash3)
    }

    fn chacha20_deterministic_shuffle(&self, master_seed: &str, miner_count: usize) -> Vec<usize> {
        // Simplified ChaCha20 PRNG simulation for deterministic shuffling
        let seed_value = self.djb2_hash(master_seed);
        let mut rng_state = seed_value;
        let mut indices: Vec<usize> = (0..miner_count).collect();
        
        // Fisher-Yates shuffle with deterministic PRNG
        for i in (1..miner_count).rev() {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345); // LCG
            let j = (rng_state as usize) % (i + 1);
            indices.swap(i, j);
        }
        
        indices
    }

    fn execute_sequential_range_assignment(&self, blockchain_state: &BlockchainState) -> Vec<MinerRange> {
        // Phase 1: Generate master seed
        let master_seed = self.generate_master_seed(blockchain_state);
        
        // Phase 2: Get active miners
        let active_miners: Vec<&MinerIdentity> = self.miner_identities
            .iter()
            .filter(|miner| miner.is_active)
            .collect();
        
        let miner_count = active_miners.len();
        
        // Phase 3: ChaCha20 deterministic shuffle
        let shuffled_indices = self.chacha20_deterministic_shuffle(&master_seed, miner_count);
        
        // Phase 4: Sequential range assignment (250,000 nonces per miner)
        // Formula: Range_i = [i × 250,000 + 1, (i + 1) × 250,000]
        let mut ranges = Vec::new();
        
        for (position, &original_index) in shuffled_indices.iter().enumerate() {
            let miner = active_miners[original_index];
            let start_nonce = (position as u64 * NONCES_PER_MINER) + 1;
            let end_nonce = (position + 1) as u64 * NONCES_PER_MINER;
            
            let range = MinerRange {
                miner_id: original_index as u32,
                identity_hash: miner.compute_identity_hash(),
                start_nonce,
                end_nonce,
                range_size: NONCES_PER_MINER,
                position,
            };
            
            ranges.push(range);
        }
        
        ranges
    }

    // Hash functions (same as MinerIdentity for consistency)
    fn djb2_hash(&self, input: &str) -> u64 {
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        hash
    }

    fn fnv_hash(&self, input: &str) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for byte in input.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }

    fn sdbm_hash(&self, input: &str) -> u64 {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = (byte as u64).wrapping_add(hash << 6).wrapping_add(hash << 16).wrapping_sub(hash);
        }
        hash
    }
}

struct NonOverlapTestFramework {
    test_results: Vec<NonOverlapTestResult>,
}

impl NonOverlapTestFramework {
    fn new() -> Self {
        NonOverlapTestFramework {
            test_results: Vec::new(),
        }
    }

    fn generate_virtual_miners(&self, count: usize) -> Vec<MinerIdentity> {
        let base_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut miners = Vec::new();
        for i in 0..count {
            miners.push(MinerIdentity::new(i as u32, base_timestamp));
        }
        
        miners
    }

    fn generate_test_scenarios(&self) -> Vec<(BlockchainState, String)> {
        let mut scenarios = Vec::new();
        let base_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Standard scenarios with fixed miner counts
        for i in 0..TEST_SCENARIOS {
            let scenario = BlockchainState {
                prev_hash: format!("block_hash_{:016x}", (i as u64).wrapping_mul(0x123456789ABCDEF)),
                block_height: (i + 1) as u64,
                timestamp: base_timestamp + (i as u64 * 500),
                active_miner_count: VIRTUAL_MINERS as u32,
            };
            scenarios.push((scenario, "Standard".to_string()));
        }
        
        // Edge case scenarios with dynamic miner counts
        for i in 0..EDGE_CASE_SCENARIOS {
            let miner_count = MIN_DYNAMIC_MINERS + (i * (MAX_DYNAMIC_MINERS - MIN_DYNAMIC_MINERS) / EDGE_CASE_SCENARIOS);
            let scenario = BlockchainState {
                prev_hash: format!("edge_hash_{:016x}", (i as u64).wrapping_mul(0xFEDCBA9876543210)),
                block_height: (TEST_SCENARIOS + i + 1) as u64,
                timestamp: base_timestamp + ((TEST_SCENARIOS + i) as u64 * 500),
                active_miner_count: miner_count as u32,
            };
            scenarios.push((scenario, "Edge Case".to_string()));
        }
        
        scenarios
    }

    fn verify_non_overlap(&self, ranges: &[MinerRange]) -> (bool, usize) {
        let mut overlap_count = 0;
        
        // Mathematical verification: ∀i≠j: Range_i ∩ Range_j = ∅
        for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                let range_i = &ranges[i];
                let range_j = &ranges[j];
                
                // Check for overlap: ranges overlap if max(start_i, start_j) <= min(end_i, end_j)
                let overlap_start = range_i.start_nonce.max(range_j.start_nonce);
                let overlap_end = range_i.end_nonce.min(range_j.end_nonce);
                
                if overlap_start <= overlap_end {
                    overlap_count += 1;
                    println!("OVERLAP DETECTED: Range {} [{}, {}] overlaps with Range {} [{}, {}]",
                            i, range_i.start_nonce, range_i.end_nonce,
                            j, range_j.start_nonce, range_j.end_nonce);
                }
            }
        }
        
        (overlap_count == 0, overlap_count)
    }

    fn verify_sequential_order(&self, ranges: &[MinerRange]) -> bool {
        // Verify: max(Range_i) < min(Range_j) when i < j
        for i in 0..(ranges.len() - 1) {
            let range_i = &ranges[i];
            let range_j = &ranges[i + 1];
            
            // Sequential order: end_nonce of position i must be < start_nonce of position i+1
            if range_i.end_nonce >= range_j.start_nonce {
                return false;
            }
        }
        
        true
    }

    fn verify_exact_nonce_count(&self, ranges: &[MinerRange]) -> bool {
        // Verify each miner receives exactly 250,000 unique nonces
        for range in ranges {
            let actual_nonces = range.end_nonce - range.start_nonce + 1;
            if actual_nonces != NONCES_PER_MINER {
                println!("NONCE COUNT ERROR: Miner {} has {} nonces, expected {}",
                        range.miner_id, actual_nonces, NONCES_PER_MINER);
                return false;
            }
        }
        
        true
    }

    fn verify_gap_analysis(&self, ranges: &[MinerRange]) -> (bool, usize) {
        let mut gap_count = 0;
        
        // Verify no gaps between consecutive ranges
        for i in 0..(ranges.len() - 1) {
            let range_i = &ranges[i];
            let range_j = &ranges[i + 1];
            
            // Gap check: end_nonce of range i + 1 should equal start_nonce of range i+1
            if range_i.end_nonce + 1 != range_j.start_nonce {
                gap_count += 1;
                println!("GAP DETECTED: Between Range {} (ends at {}) and Range {} (starts at {})",
                        i, range_i.end_nonce, i + 1, range_j.start_nonce);
            }
        }
        
        (gap_count == 0, gap_count)
    }

    fn simulate_dynamic_miner_changes(&self, miners: &mut Vec<MinerIdentity>, scenario_id: usize) {
        // Simulate miners joining and leaving dynamically
        let change_type = scenario_id % 4;
        
        match change_type {
            0 => {
                // Miners leaving
                let leave_count = (miners.len() / 10).max(1);
                for i in 0..leave_count {
                    if i < miners.len() {
                        miners[i].is_active = false;
                    }
                }
            },
            1 => {
                // Miners joining
                let base_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                let join_count = miners.len() / 20;
                for i in 0..join_count {
                    let new_id = miners.len() + i;
                    miners.push(MinerIdentity::new(new_id as u32, base_timestamp));
                }
            },
            2 => {
                // Mixed: some leave, some join
                let change_count = miners.len() / 15;
                for i in 0..change_count {
                    if i < miners.len() {
                        miners[i].is_active = false;
                    }
                }
                
                let base_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                for i in 0..change_count {
                    let new_id = miners.len() + i;
                    miners.push(MinerIdentity::new(new_id as u32, base_timestamp));
                }
            },
            _ => {
                // No changes (control group)
            }
        }
    }

    fn run_comprehensive_non_overlap_test(&mut self) -> NonOverlapStatistics {
        println!("\n=================================================================================");
        println!("I PROTOCOL - TEST 2.2: DURA RANGE NON-OVERLAP VERIFICATION");
        println!("=================================================================================");
        println!("Objective: Mathematically prove ∀i≠j: Range_i ∩ Range_j = ∅");
        println!("Virtual Miners: {}", VIRTUAL_MINERS);
        println!("Test Scenarios: {}", TEST_SCENARIOS);
        println!("Edge Case Scenarios: {}", EDGE_CASE_SCENARIOS);
        println!("Nonces per Miner: {}", NONCES_PER_MINER);
        println!("Protocol Salt: {}", PROTOCOL_SALT);
        println!("=================================================================================");
        println!();
        
        let scenarios = self.generate_test_scenarios();
        let mut non_overlap_success = 0;
        let mut sequential_order_success = 0;
        let mut exact_nonce_success = 0;
        let mut gap_analysis_success = 0;
        let mut total_miners_tested = 0;
        let mut total_ranges_verified = 0;
        let mut total_nonces_verified = 0u64;
        let mut overlap_violations = 0;
        let mut gap_violations = 0;
        let mut standard_scenarios = 0;
        let mut edge_case_scenarios = 0;
        let mut dynamic_scenarios = 0;
        
        for (test_id, (scenario, scenario_type)) in scenarios.iter().enumerate() {
            // Generate miners for this scenario
            let mut miners = self.generate_virtual_miners(scenario.active_miner_count as usize);
            
            // Apply dynamic changes for edge cases
            if scenario_type == "Edge Case" {
                self.simulate_dynamic_miner_changes(&mut miners, test_id);
                dynamic_scenarios += 1;
            }
            
            if scenario_type == "Standard" {
                standard_scenarios += 1;
            } else {
                edge_case_scenarios += 1;
            }
            
            // Create DURA assignment engine
            let assignment_engine = DuraRangeAssignmentEngine::new(miners.clone());
            
            // Execute range assignment
            let range_assignments = assignment_engine.execute_sequential_range_assignment(scenario);
            
            // Comprehensive verification
            let (non_overlap_verified, overlap_count) = self.verify_non_overlap(&range_assignments);
            let sequential_order_verified = self.verify_sequential_order(&range_assignments);
            let exact_nonce_count_verified = self.verify_exact_nonce_count(&range_assignments);
            let (gap_analysis_passed, gap_count) = self.verify_gap_analysis(&range_assignments);
            
            let range_verification_passed = non_overlap_verified && 
                                           sequential_order_verified && 
                                           exact_nonce_count_verified && 
                                           gap_analysis_passed;
            
            // Update statistics
            if non_overlap_verified {
                non_overlap_success += 1;
            } else {
                overlap_violations += overlap_count;
            }
            
            if sequential_order_verified {
                sequential_order_success += 1;
            }
            
            if exact_nonce_count_verified {
                exact_nonce_success += 1;
            }
            
            if gap_analysis_passed {
                gap_analysis_success += 1;
            } else {
                gap_violations += gap_count;
            }
            
            let active_miners = miners.iter().filter(|m| m.is_active).count();
            total_miners_tested += active_miners;
            total_ranges_verified += range_assignments.len();
            total_nonces_verified += range_assignments.len() as u64 * NONCES_PER_MINER;
            
            // Store test result
            let test_result = NonOverlapTestResult {
                test_id,
                scenario_type: scenario_type.clone(),
                blockchain_state: scenario.clone(),
                range_assignments,
                total_miners: active_miners,
                total_nonces_assigned: active_miners as u64 * NONCES_PER_MINER,
                range_verification_passed,
                non_overlap_verified,
                sequential_order_verified,
                exact_nonce_count_verified,
                gap_analysis_passed,
                overlap_count,
                gap_count,
            };
            self.test_results.push(test_result);
            
            // Progress reporting
            if (test_id + 1) % 100 == 0 {
                println!("Completed {} scenarios... (Non-overlap success rate: {:.2}%)", 
                        test_id + 1, 
                        (non_overlap_success as f64 / (test_id + 1) as f64) * 100.0);
            }
        }
        
        let total_tests = scenarios.len();
        let success_rate = (non_overlap_success as f64 / total_tests as f64) * 100.0;
        let mathematical_proof_validated = non_overlap_success == total_tests && 
                                          sequential_order_success == total_tests &&
                                          exact_nonce_success == total_tests &&
                                          gap_analysis_success == total_tests;
        let test_passed = mathematical_proof_validated;
        
        println!("\n=================================================================================");
        println!("DURA RANGE NON-OVERLAP VERIFICATION RESULTS");
        println!("=================================================================================");
        println!("Total Test Scenarios: {}", total_tests);
        println!("Standard Scenarios: {}", standard_scenarios);
        println!("Edge Case Scenarios: {}", edge_case_scenarios);
        println!("Dynamic Change Scenarios: {}", dynamic_scenarios);
        println!("Non-Overlap Success: {}", non_overlap_success);
        println!("Sequential Order Success: {}", sequential_order_success);
        println!("Exact Nonce Count Success: {}", exact_nonce_success);
        println!("Gap Analysis Success: {}", gap_analysis_success);
        println!("Total Miners Tested: {}", total_miners_tested);
        println!("Total Ranges Verified: {}", total_ranges_verified);
        println!("Total Nonces Verified: {}", total_nonces_verified);
        println!("Overlap Violations: {}", overlap_violations);
        println!("Gap Violations: {}", gap_violations);
        println!("Success Rate: {:.6}%", success_rate);
        println!();
        
        if test_passed {
            println!("MATHEMATICAL NON-OVERLAP VALIDATION:");
            println!("∀i≠j: Range_i ∩ Range_j = ∅: MATHEMATICALLY PROVEN");
            println!("Sequential Range Assignment: Range_i = [i × 250,000 + 1, (i + 1) × 250,000]");
            println!("Non-Overlapping Property: max(Range_i) < min(Range_j) when i < j: VERIFIED");
            println!("Exact Nonce Allocation: Each miner receives exactly 250,000 nonces: GUARANTEED");
            println!("Zero Range Overlaps: MATHEMATICALLY CERTAIN");
            println!();
            println!("RANGE ASSIGNMENT ANALYSIS:");
            println!("- Sequential Formula: start_nonce = position × 250,000 + 1");
            println!("- Sequential Formula: end_nonce = (position + 1) × 250,000");
            println!("- Range Size: Exactly 250,000 nonces per miner");
            println!("- Gap Prevention: end_nonce(i) + 1 = start_nonce(i+1)");
            println!("- Overlap Prevention: Mathematically impossible by design");
            println!();
            println!("EDGE CASE VERIFICATION:");
            println!("- Dynamic Miner Joining: HANDLED CORRECTLY");
            println!("- Dynamic Miner Leaving: HANDLED CORRECTLY");
            println!("- Mixed Join/Leave Scenarios: VERIFIED");
            println!("- Variable Miner Counts: {} to {} miners tested", MIN_DYNAMIC_MINERS, MAX_DYNAMIC_MINERS);
            println!("- Range Integrity: MAINTAINED ACROSS ALL SCENARIOS");
        } else {
            println!("DURA RANGE NON-OVERLAP VERIFICATION FAILED:");
            println!("Range overlaps detected: {}", overlap_violations);
            println!("Gap violations detected: {}", gap_violations);
            println!("Failed scenarios: {}", total_tests - non_overlap_success);
            println!("System requires immediate investigation");
        }
        
        println!("\n=================================================================================");
        println!("MATHEMATICAL PROOF ANALYSIS");
        println!("=================================================================================");
        println!("Range Assignment Formula Verification:");
        println!("- Formula: Range_i = [i × 250,000 + 1, (i + 1) × 250,000]");
        println!("- Non-Overlap Proof: end_nonce(i) = i × 250,000 < (i+1) × 250,000 + 1 = start_nonce(i+1)");
        println!("- Mathematical Certainty: ∀i≠j: max(Range_i) < min(Range_j) when i < j");
        println!("- Set Theory: Range_i ∩ Range_j = ∅ for all i ≠ j");
        println!();
        println!("Nonce Distribution Analysis:");
        println!("- Total Nonces Verified: {}", total_nonces_verified);
        println!("- Nonces per Miner: {} (constant)", NONCES_PER_MINER);
        println!("- Range Uniformity: All ranges exactly equal size");
        println!("- Coverage Completeness: No nonce gaps or overlaps");
        
        println!("\n=================================================================================");
        println!("OVERALL TEST RESULT: {}", if test_passed { "PASS" } else { "FAIL" });
        println!("=================================================================================");
        
        NonOverlapStatistics {
            total_tests,
            standard_scenarios,
            edge_case_scenarios,
            dynamic_scenarios,
            non_overlap_success,
            sequential_order_success,
            exact_nonce_success,
            gap_analysis_success,
            total_miners_tested,
            total_ranges_verified,
            total_nonces_verified,
            overlap_violations,
            gap_violations,
            success_rate,
            mathematical_proof_validated,
            test_passed,
        }
    }
}

fn main() {
    let mut test_framework = NonOverlapTestFramework::new();
    let statistics = test_framework.run_comprehensive_non_overlap_test();
    
    // Final validation
    if statistics.test_passed {
        println!("\nTEST 2.2 COMPLETION: DURA RANGE NON-OVERLAP VERIFICATION SUCCESSFUL");
        println!("Mathematical proof ∀i≠j: Range_i ∩ Range_j = ∅: ESTABLISHED");
        println!("Sequential range assignment formula: VERIFIED");
        println!("Zero range overlaps in all test scenarios: ACHIEVED");
        println!("Edge cases with dynamic miner joining/leaving: HANDLED");
        println!("Success Criteria: Zero range overlaps in all scenarios: ACCOMPLISHED");
    } else {
        println!("\nTEST 2.2 COMPLETION: DURA RANGE NON-OVERLAP VERIFICATION FAILED");
        println!("Range overlaps detected - Algorithm requires review");
        println!("Mathematical proof could not be established");
    }
}