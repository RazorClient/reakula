use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};
use alloy_helper::primitives::B256;

use crate::{phase0::primitives::{BLSPubkey, Epoch, Gwei}, common::CustomPrimitiveType};

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
pub struct Validator {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: CustomPrimitiveType<B256>,
    pub effective_balance: Gwei,
    pub slashed: CustomPrimitiveType<bool>,
    pub activation_eligibility_epoch: Epoch,
    pub activation_epoch: Epoch,
    pub exit_epoch: Epoch,
    pub withdrawable_epoch: Epoch,
}