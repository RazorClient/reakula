use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::macros::impl_safe_arith;

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
pub struct Slot {
    value: u64,
}

impl From<u64> for Slot {
    fn from(value: u64) -> Self {
        Slot { value }
    }
}

impl From<Slot> for u64 {
    fn from(slot: Slot) -> u64 {
        slot.value
    }
}

impl From<usize> for Slot {
    fn from(value: usize) -> Self {
        Slot {
            value: value as u64,
        }
    }
}

impl From<Slot> for usize {
    fn from(slot: Slot) -> usize {
        slot.value as usize
    }
}

impl_safe_arith!(Slot, Slot);
impl_safe_arith!(Slot, u64);

impl Slot {

    /// Creates a new Slot.
    pub const fn new(value: u64) -> Self {
        Slot { value }
    }

    /// Returns the value of the Slot.
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn max_value() -> Self {
        Slot { value: u64::MAX }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }

    /// Get the value of the Slot as bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.put_u64(self.value);
        bytes
    }

    /// Create a Slot from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = bytes;
        Slot {
            value: buf.get_u64(),
        }
    }
}