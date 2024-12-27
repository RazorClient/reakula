use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::phase0::primitives::{Slot, ValidatorIndex};

use super::AttestationData;

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
pub struct PendingAttestation {
// pub struct PendingAttestation<P: Preset> {
    // TODO: MaxValidatorPerCommittee can defer depending on the epoch and the number of avalable validators. So write a struct that can check ValidatorIndex with MaxValidatorPerCommittee
    // pub aggregation_bits: BitList<P::MaxValidatorsPerCommittee>,
    pub aggregation_bits: ValidatorIndex,
    pub data: AttestationData,
    pub inclusion_delay: Slot,
    pub proposer_index: ValidatorIndex,
}