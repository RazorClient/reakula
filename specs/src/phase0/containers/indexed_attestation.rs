use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::phase0::primitives::{BLSSignature, ValidatorIndex};

use super::AttestationData;

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
pub struct IndexedAttestation {
// pub struct IndexedAttestation<P: Preset> {
    // TODO: MaxValidatorPerCommittee can defer depending on the epoch and the number of avalable validators. So write a struct that can check ValidatorIndex with MaxValidatorPerCommittee
    // pub attesting_indices: ContiguousList<ValidatorIndex, P::MaxValidatorsPerCommittee>,
    pub attesting_indices: ValidatorIndex,
    pub data: AttestationData,
    pub signature: BLSSignature,
}