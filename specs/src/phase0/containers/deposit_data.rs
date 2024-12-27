use alloy_helper::primitives::B256;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::{common::CustomPrimitiveType, phase0::primitives::{BLSPubkey, BLSSignature, Gwei}};

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
pub struct DepositData {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: CustomPrimitiveType<B256>,
    pub amount: Gwei,
    pub signature: BLSSignature
}