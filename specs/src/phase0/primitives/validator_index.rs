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
pub struct ValidatorIndex {
    value: u64,
}

impl From<u64> for ValidatorIndex {
    fn from(value: u64) -> Self {
        ValidatorIndex { value }
    }
}

impl From<ValidatorIndex> for u64 {
    fn from(index: ValidatorIndex) -> u64 {
        index.value
    }
}

impl From<usize> for ValidatorIndex {
    fn from(value: usize) -> Self {
        ValidatorIndex {
            value: value as u64,
        }
    }
}

impl From<ValidatorIndex> for usize {
    fn from(index: ValidatorIndex) -> usize {
        index.value as usize
    }
}

impl_safe_arith!(ValidatorIndex, ValidatorIndex);
impl_safe_arith!(ValidatorIndex, u64);

impl ValidatorIndex {

    /// Creates a new ValidatorIndex.
    pub const fn new(value: u64) -> Self {
        ValidatorIndex { value }
    }

    /// Returns the value of the ValidatorIndex.
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn max_value() -> Self {
        ValidatorIndex { value: u64::MAX }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }
}