use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

#[derive(
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
pub struct Domain {
    value: B256
}

impl From<B256> for Domain {
    fn from(value: B256) -> Self {
        Domain { value }
    }
}

impl From<Domain> for B256 {
    fn from(domain: Domain) -> B256 {
        domain.value
    }
}

impl Domain {
    /// Creates a new Root.
    pub const fn new(value: B256) -> Self {
        Domain { value }
    }

    pub fn value(&self) -> B256 {
        self.value
    }

    pub fn zero() -> Self {
        Domain::new(B256::ZERO)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Domain::new(B256::from_slice(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
}