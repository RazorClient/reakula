use alloy_primitives::B256;
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
pub struct Root {
    value: B256
}

impl From<B256> for Root {
    fn from(value: B256) -> Self {
        Root { value }
    }
}

impl From<Root> for B256 {
    fn from(root: Root) -> B256 {
        root.value
    }
}

impl Root {
    /// Creates a new Root.
    pub const fn new(value: B256) -> Self {
        Root { value }
    }

    pub fn value(&self) -> B256 {
        self.value
    }

    pub fn zero() -> Self {
        Root::new(B256::ZERO)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Root::new(B256::from_slice(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
}