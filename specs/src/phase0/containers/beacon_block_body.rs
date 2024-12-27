use alloy_helper::primitives::B256;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::{common::CustomPrimitiveType, phase0::primitives::BLSSignature};

use super::{Attestation, AttesterSlashing, Deposit, Eth1Data, ProposerSlashing, SignedVoluntaryExit};

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    SszbEncode,
    SszbDecode,
)]
#[serde(deny_unknown_fields)]
pub struct BeaconBlockBody {
// pub struct BeaconBlockBody<P: Preset> {
    pub randao_reveal: BLSSignature,
    pub eth1_data: Eth1Data,
    pub graffiti: CustomPrimitiveType<B256>,
    // pub proposer_slashings: List<ProposerSlashing, P::MaxProposerSlashings>,
    pub proposer_slashings: ProposerSlashing,
    // pub attester_slashings: List<AttesterSlashing, P::MaxAttesterSlashings>,
    pub attester_slashings: AttesterSlashing,
    // pub attestations: List<Attestation, P::MaxAttestations>,
    pub attestations: Attestation,
    // pub deposits: List<Deposit, P::MaxDeposits>,
    pub deposits: Deposit,
    // pub voluntary_exits: List<SignedVoluntaryExit, P::MaxVoluntaryExits>,
    pub voluntary_exits: SignedVoluntaryExit
}