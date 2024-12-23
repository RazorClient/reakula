use alloy_primitives::aliases::B32;
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
pub struct DomainType {
    value: B32
}

impl From<B32> for DomainType {
    fn from(value: B32) -> Self {
        DomainType { value }
    }
}

impl From<DomainType> for B32 {
    fn from(domain_type: DomainType) -> B32 {
        domain_type.value
    }
}

impl DomainType {
    /// Creates a new Root.
    pub const fn new(value: B32) -> Self {
        DomainType { value }
    }

    pub fn value(&self) -> B32 {
        self.value
    }

    pub fn zero() -> Self {
        DomainType::new(B32::ZERO)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        DomainType::new(B32::from_slice(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
}