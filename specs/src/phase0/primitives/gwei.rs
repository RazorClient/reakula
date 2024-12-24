use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::macros::impl_safe_arith;

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
pub struct Gwei {
    value: u64,
}

impl From<u64> for Gwei {
    fn from(value: u64) -> Self {
        Gwei { value }
    }
}

impl From<Gwei> for u64 {
    fn from(gwei: Gwei) -> u64 {
        gwei.value
    }
}

impl From<usize> for Gwei {
    fn from(value: usize) -> Self {
        Gwei {
            value: value as u64,
        }
    }
}

impl From<Gwei> for usize {
    fn from(gewi: Gwei) -> usize {
        gewi.value as usize
    }
}

impl_safe_arith!(Gwei, Gwei);
impl_safe_arith!(Gwei, u64);

impl Gwei {

    /// Creates a new Gewi.
    pub const fn new(value: u64) -> Self {
        Gwei { value }
    }

    /// Returns the value of the Gewi.
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn max_value() -> Self {
        Gwei { value: u64::MAX }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }
}