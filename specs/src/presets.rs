use std::hash::Hash;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use typenum::NonZero;


pub trait Preset: Copy + Eq + Ord + Hash + Default + Debug + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static {

    // Phase 0 types
    type MaxValidatorsPerCommittee: Serialize + for<'a> Deserialize<'a>
        + NonZero
        + Eq
        + Ord
        + Debug
        + Send
        + Sync;

    // Phase 0
    const MAX_COMMITTEES_PER_SLOT: u64 = 64;
    const TARGET_COMMITTEE_SIZE: u64 = 128;
    const SHUFFLE_ROUND_COUNT: u64 = 90;
    const HYSTERESIS_QUOTIENT: u64 = 4;
    const HYSTERESIS_DOWNWARD_MULTIPLIER: u64 = 1;
    const HYSTERESIS_UPWARD_MULTIPLIER: u64 = 5;
    const SAFE_SLOTS_TO_UPDATE_JUSTIFIED: u64 = 8;
    const MIN_DEPOSIT_AMOUNT: u64 = 1_000_000_000;
    const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000;
    const EFFECTIVE_BALANCE_INCREMENT: u64 = 1_000_000_000;
    const MIN_ATTESTATION_INCLUSION_DELAY: u64 = 1;
    const SLOTS_PER_EPOCH: u64 = 32;
    const MIN_SEED_LOOKAHEAD: u64 = 1;
    const MAX_SEED_LOOKAHEAD: u64 = 4;
    const EPOCHS_PER_ETH1_VOTING_PERIOD: u64 = 64;
    const SLOTS_PER_HISTORICAL_ROOT: u64 = 8_192;
    const MIN_EPOCHS_TO_INACTIVITY_PENALTY: u64 = 4;
    const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 65536;
    const EPOCHS_PER_SLASHINGS_VECTOR: u64 = 8192;
    const HISTORICAL_ROOTS_LIMIT: u64 = 16777216;
    const VALIDATOR_REGISTRY_LIMIT: u64 = 1099511627776;
    const BASE_REWARD_FACTOR: u64 = 64;
    const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 512;
    const PROPOSER_REWARD_QUOTIENT: u64 = 8;
    const INACTIVITY_PENALTY_QUOTIENT: u64 = 67108864;
    const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 128;
    const PROPORTIONAL_SLASHING_MULTIPLIER: u64 = 1;
    const MAX_PROPOSER_SLASHINGS: u64 = 16;
    const MAX_ATTESTER_SLASHINGS: u64 = 2;
    const MAX_ATTESTATIONS: u64 = 128;
    const MAX_DEPOSITS: u64 = 16;
    const MAX_VOLUNTARY_EXITS: u64 = 16;
}