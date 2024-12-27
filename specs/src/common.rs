use serde::{Deserialize, Serialize};
use sszb::{SszDecode, SszEncode};
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
pub struct CustomPrimitiveType<T: CommonType + SszEncode + SszDecode> {
    value: T
}

pub trait CommonType {}

impl<N> CommonType for N {}