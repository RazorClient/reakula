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
pub struct Gewi {
    value: u64,
}

impl From<u64> for Gewi {
    fn from(value: u64) -> Self {
        Gewi { value }
    }
}

impl From<Gewi> for u64 {
    fn from(gewi: Gewi) -> u64 {
        gewi.value
    }
}

impl From<usize> for Gewi {
    fn from(value: usize) -> Self {
        Gewi {
            value: value as u64,
        }
    }
}

impl From<Gewi> for usize {
    fn from(gewi: Gewi) -> usize {
        gewi.value as usize
    }
}

impl_safe_arith!(Gewi, Gewi);
impl_safe_arith!(Gewi, u64);

impl Gewi {

    /// Creates a new Gewi.
    pub const fn new(value: u64) -> Self {
        Gewi { value }
    }

    /// Returns the value of the Gewi.
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn max_value() -> Self {
        Gewi { value: u64::MAX }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }
}