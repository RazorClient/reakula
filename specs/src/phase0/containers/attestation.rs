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
pub struct Attestation {
// pub struct Attestation<P: Preset> {
    // pub aggregation_bits: BitList<P::MaxValidatorsPerCommittee>,
    pub aggregation_bits: ValidatorIndex,
    pub data: AttestationData,
    pub signature: BLSSignature
}