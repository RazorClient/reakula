use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate:: phase0::primitives::{Root, Slot, ValidatorIndex};

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
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body_root: Root
}