use alloy_helper::B384;
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
pub struct BLSSignature {
    value: B384
}

impl From<B384> for BLSSignature {
    fn from(value: B384) -> Self {
        BLSSignature { value }
    }
}

impl From<BLSSignature> for B384 {
    fn from(bls_signature: BLSSignature) -> B384 {
        bls_signature.value
    }
}

impl BLSSignature {
    /// Creates a new Root.
    pub const fn new(value: B384) -> Self {
        BLSSignature { value }
    }

    pub fn value(&self) -> B384 {
        self.value
    }

    pub fn zero() -> Self {
        BLSSignature::new(B384::ZERO)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        BLSSignature::new(B384::from_slice(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
}