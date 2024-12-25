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
pub struct CommitteeIndex {
    value: u64,
}

impl From<u64> for CommitteeIndex {
    fn from(value: u64) -> Self {
        CommitteeIndex { value }
    }
}

impl From<CommitteeIndex> for u64 {
    fn from(index: CommitteeIndex) -> u64 {
        index.value
    }
}

impl From<usize> for CommitteeIndex {
    fn from(value: usize) -> Self {
        CommitteeIndex {
            value: value as u64,
        }
    }
}

impl From<CommitteeIndex> for usize {
    fn from(index: CommitteeIndex) -> usize {
        index.value as usize
    }
}

impl_safe_arith!(CommitteeIndex, CommitteeIndex);
impl_safe_arith!(CommitteeIndex, u64);

impl CommitteeIndex {

    /// Creates a new CommitteeIndex.
    pub const fn new(value: u64) -> Self {
        CommitteeIndex { value }
    }

    /// Returns the value of the CommitteeIndex.
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn max_value() -> Self {
        CommitteeIndex { value: u64::MAX }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }
}