use super::primitives::{DomainType, Epoch, Slot, Version};
use alloy_helper::primitives::fixed_bytes;
use byteorder::LittleEndian;


pub const GENESIS_SLOT: Slot = Slot::new(0);
pub const GENESIS_EPOCH: Epoch = Epoch::new(0);
pub const FAR_FUTURE_EPOCH: Epoch = Epoch::new(u64::MAX);
pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub const JUSTIFICATION_BITS_LENGTH: usize = 4;

pub type ENDIANNESS = LittleEndian;

pub const BLS_WITHDRAWAL_PREFIX: u8 = 0x00;
pub const ETH1_ADDRESS_WITHDRAWAL_PREFIX: u8 = 0x01;

pub const DOMAIN_BEACON_PROPOSER: DomainType = DomainType::new(fixed_bytes!("00000000"));
pub const DOMAIN_BEACON_ATTESTER: DomainType = DomainType::new(fixed_bytes!("01000000"));
pub const DOMAIN_RANDAO: DomainType = DomainType::new(fixed_bytes!("02000000"));
pub const DOMAIN_DEPOSIT: DomainType = DomainType::new(fixed_bytes!("03000000"));
pub const DOMAIN_VOLUNTARY_EXIT: DomainType = DomainType::new(fixed_bytes!("04000000"));
pub const DOMAIN_SELECTION_PROOF: DomainType = DomainType::new(fixed_bytes!("05000000"));
pub const DOMAIN_AGGREGATE_AND_PROOF: DomainType = DomainType::new(fixed_bytes!("06000000"));
pub const DOMAIN_APPLICATION_MASK: DomainType = DomainType::new(fixed_bytes!("00000001"));

pub const MAX_COMMITTEES_PER_SLOT: u64 = 64;
pub const TARGET_COMMITTEE_SIZE: u64 = 128;
pub const MAX_VALIDATORS_PER_COMMITTEE: u64 = 2048;
pub const SHUFFLE_ROUND_COUNT: u8 = 90;
pub const HYSTERESIS_QUOTIENT: u64 = 4;
pub const HYSTERESIS_DOWNWARD_MULTIPLIER: u64 = 1;
pub const HYSTERESIS_UPWARD_MULTIPLIER: u64 = 5;

pub const MIN_DEPOSIT_AMOUNT: u64 = 1_000_000_000;
pub const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000;
pub const EFFECTIVE_BALANCE_INCREMENT: u64 = 1_000_000_000;

pub const MIN_ATTESTATION_INCLUSION_DELAY: u64 = 1;
pub const SLOTS_PER_EPOCH: u64 = 32;
pub const MIN_SEED_LOOKAHEAD: u64 = 1;
pub const MAX_SEED_LOOKAHEAD: u64 = 4;
pub const MIN_EPOCHS_TO_INACTIVITY_PENALTY: u64 = 4;
pub const EPOCHS_PER_ETH1_VOTING_PERIOD: u64 = 64;
pub const SLOTS_PER_HISTORICAL_ROOT: u64 = 8_192;

pub const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 65_536;
pub const EPOCHS_PER_SLASHINGS_VECTOR: u64 = 8_192;
pub const HISTORICAL_ROOTS_LIMIT: u64 = 16_777_216;
pub const VALIDATOR_REGISTRY_LIMIT: u64 = 1_099_511_627_776;

pub const BASE_REWARD_FACTOR: u64 = 64;
pub const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 512;
pub const PROPOSER_REWARD_QUOTIENT: u64 = 8;
pub const INACTIVITY_PENALTY_QUOTIENT: u64 = 67_108_864;
pub const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 128;
pub const PROPORTIONAL_SLASHING_MULTIPLIER: u64 = 1;

pub const MAX_PROPOSER_SLASHINGS: u64 = 16;
pub const MAX_ATTESTER_SLASHINGS: u64 = 2;
pub const MAX_ATTESTATIONS: u64 = 128;
pub const MAX_DEPOSITS: u64 = 16;
pub const MAX_VOLUNTARY_EXITS: u64 = 16;

pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: u64 = 16_384;
pub const MIN_GENESIS_TIME: u64 = 1606824000;
pub const GENESIS_FORK_VERSION: Version = Version::new(fixed_bytes!("00000000"));
pub const GENESIS_DELAY: u64 = 604800;

pub const SECONDS_PER_SLOT: u64 = 12;
pub const SECONDS_PER_ETH1_BLOCK: u64 = 14;
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: u64 = 256;
pub const SHARD_COMMITTEE_PERIOD: u64 = 256;
pub const ETH1_FOLLOW_DISTANCE: u64 = 2048;

pub const EJECTION_BALANCE: u64 = 16_000_000_000;
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
pub const CHURN_LIMIT_QUOTIENT: u64 = 65_536;