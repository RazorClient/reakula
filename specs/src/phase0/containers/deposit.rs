use alloy_helper::primitives::B256;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::common::CustomPrimitiveType;

use super::DepositData;

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
pub struct Deposit {
// pub struct Deposit<P: Preset> {
    // pub proof: Vector<B256, P:DepositContractTreeDepth>,
    pub proof: CustomPrimitiveType<B256>,
    pub data: DepositData
}