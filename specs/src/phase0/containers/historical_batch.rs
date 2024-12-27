use serde::{Deserialize, Serialize};
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};
use bytes::{Buf, BufMut};

use crate::phase0::primitives::Root;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    SszbEncode,
    SszbDecode,
)]
#[serde(deny_unknown_fields)]
pub struct HistoricalBatch {
// pub struct HistoricalBatch<P: Preset> {
    // TODO: This should be a Vector<Root, SLOTS_PER_HISTORICAL_ROOT>
    // pub block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub block_roots: Root,
    // pub state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub state_roots: Root,
}