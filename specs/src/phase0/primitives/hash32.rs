use alloy_helper::primitives::B256;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    SszbEncode,
    SszbDecode,
)]
pub struct Hash32 {
    value: B256
}

impl From<B256> for Hash32 {
    fn from(value: B256) -> Self {
        Hash32 { value }
    }
}

impl From<Hash32> for B256 {
    fn from(hash32: Hash32) -> B256 {
        hash32.value
    }
}

impl Hash32 {
    /// Creates a new Root.
    pub const fn new(value: B256) -> Self {
        Hash32 { value }
    }

    pub fn value(&self) -> B256 {
        self.value
    }

    pub fn zero() -> Self {
        Hash32::new(B256::ZERO)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Hash32::new(B256::from_slice(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
}