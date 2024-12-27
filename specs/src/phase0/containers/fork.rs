use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::phase0::primitives::{Epoch, Version};

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
pub struct Fork {
    pub previous_version: Version,
    pub current_version: Version,
    pub epoch: Epoch
}