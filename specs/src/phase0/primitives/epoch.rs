use safe_arith::SafeArith;
use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::macros::impl_safe_arith;
use crate::phase0::primitives::slot::Slot;

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
pub struct Epoch {
    value: u64,
}

impl From<u64> for Epoch {
    fn from(value: u64) -> Self {
        Epoch { value }
    }
}

impl From<Epoch> for u64 {
    fn from(epoch: Epoch) -> u64 {
        epoch.value
    }
}

impl From<usize> for Epoch {
    fn from(value: usize) -> Self {
        Epoch {
            value: value as u64,
        }
    }
}

impl From<Epoch> for usize {
    fn from(epoch: Epoch) -> usize {
        epoch.value as usize
    }
}

impl_safe_arith!(Epoch, Epoch);
impl_safe_arith!(Epoch, u64);

impl Epoch {

    /// Creates a new Epoch.
    pub const fn new(value: u64) -> Self {
        Epoch { value }
    }

    /// Returns the value of the Epoch.
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn max_value() -> Self {
        Epoch { value: u64::MAX }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }

    /// Get the value of the Epoch as bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.put_u64(self.value);
        bytes
    }

    /// Create a Epoch from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = bytes;
        Epoch {
            value: buf.get_u64(),
        }
    }

    /// The first slot in the epoch.
    pub fn start_slot(self, slots_per_epoch: u64) -> Slot {
        Slot::from(self.value.saturating_mul(slots_per_epoch))
    }

    /// The last slot in the epoch.
    pub fn end_slot(self, slots_per_epoch: u64) -> Slot {
        Slot::from(
            self.value
                .saturating_mul(slots_per_epoch)
                .saturating_add(slots_per_epoch.saturating_sub(1)),
        )
    }

    /// Position of some slot inside an epoch, if any.
    ///
    /// E.g., the first `slot` in `epoch` is at position `0`.
    pub fn position(self, slot: Slot, slots_per_epoch: u64) -> Option<usize> {
        let start = self.start_slot(slots_per_epoch);
        let end = self.end_slot(slots_per_epoch);

        if slot >= start && slot <= end {
            slot.as_usize().checked_sub(start.as_usize())
        } else {
            None
        }
    }

    pub fn slot_iter(&self, slots_per_epoch: u64) -> SlotIter {
        SlotIter {
            current_iteration: 0,
            epoch: self,
            slots_per_epoch,
        }
    }
}

pub struct SlotIter<'a> {
    current_iteration: u64,
    epoch: &'a Epoch,
    slots_per_epoch: u64,
}

impl Iterator for SlotIter<'_> {
    type Item = Slot;

    fn next(&mut self) -> Option<Slot> {
        if self.current_iteration >= self.slots_per_epoch {
            None
        } else {
            let start_slot = self.epoch.start_slot(self.slots_per_epoch);
            let previous = self.current_iteration;
            self.current_iteration = self.current_iteration.checked_add(1)?;
            start_slot.safe_add(previous).ok()
        }
    }
}