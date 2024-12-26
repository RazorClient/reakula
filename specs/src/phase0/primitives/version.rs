use alloy_helper::primitives::aliases::B32;
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
pub struct Version {
    value: B32
}

impl From<B32> for Version {
    fn from(value: B32) -> Self {
        Version { value }
    }
}

impl From<Version> for B32 {
    fn from(version: Version) -> B32 {
        version.value
    }
}

impl Version {
    /// Creates a new Root.
    pub const fn new(value: B32) -> Self {
        Version { value }
    }

    pub fn value(&self) -> B32 {
        self.value
    }

    pub fn zero() -> Self {
        Version::new(B32::ZERO)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Version::new(B32::from_slice(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
}