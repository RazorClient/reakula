use alloy_helper::primitives::B256;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::common::CustomPrimitiveType;

use super::{containers::{BeaconBlockHeader, Checkpoint, Eth1Data, Fork, PendingAttestation, Validator}, primitives::{Gwei, Root, Slot}};


#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    SszbEncode,
    SszbDecode,
)]
#[serde(deny_unknown_fields)]
pub struct BeaconState {
// pub struct BeaconState<P: Preset> {
    // >> Versioning
    pub genesis_time: CustomPrimitiveType<u64>,
    pub genesis_validators_root: CustomPrimitiveType<B256>,
    pub slot: Slot,
    pub fork: Fork,

    // >> History
    pub latest_block_header: BeaconBlockHeader,
    // pub block_roots: Vector<Root, P::SlotsPerHistoricalRoot>,
    pub block_roots: Root,
    // pub state_roots: Vector<Root, P::SlotsPerHistoricalRoot>,
    pub state_roots: Root,
    // pub historical_roots: Vector<Root, P::HistoricalRootsLimit>,
    pub historical_roots: Root,

    // >> Eth1
    pub eth1_data: Eth1Data,
    // pub eth1_data_votes: List<Eth1Data, EPOCHS_PER_ETH1_VOTING_PERIOD * SLOTS_PER_EPOCH>,
    pub eth1_data_votes: Eth1Data,
    pub eth1_deposit_index: CustomPrimitiveType<u64>,

    // >> Registry
    // pub validators: List<Validator, VALIDATOR_REGISTRY_LIMIT>,
    pub validators: Validator,
    // pub balances: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    pub balances: Gwei,

    // >> Randomness
    // pub randao_mixes: Vector<Bytes32, EPOCHS_PER_HISTORICAL_VECTOR>,
    pub randao_mixes: CustomPrimitiveType<B256>,

    // >> Slashings
    // pub slashings: Vector<Gwei, EPOCHS_PER_SLASHINGS_VECTOR>,
    pub slashing: Gwei,

    // >> Attestations
    // pub previous_epoch_attestations: List<PendingAttestation, MAX_ATTESTATIONS * SLOTS_PER_EPOCH>,
    pub previous_epoch_attestations: PendingAttestation,
    // pub current_epoch_attestations: List<PendingAttestation, MAX_ATTESTATIONS * SLOTS_PER_EPOCH>,
    pub current_epoch_attestations: PendingAttestation,

    // >> Finality
    // pub justification_bits: BitVector<JustificationBitsLength>,
    pub justification_bits: CustomPrimitiveType<u64>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
}