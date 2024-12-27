use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::{common::CustomPrimitiveType, phase0::primitives::{Hash32, Root}};

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
pub struct Eth1Data {
    pub deposit_root: Root,
    pub deposit_count: CustomPrimitiveType<u64>,
    pub block_hash: Hash32,
}