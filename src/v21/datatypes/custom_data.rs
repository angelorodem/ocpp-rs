//! Vendor extension envelope (`customData`).
//!
//! Part 3 schemas intentionally omit `additionalProperties: false` so vendors
//! may attach arbitrary JSON properties alongside required `vendorId`.

use alloc::{collections::BTreeMap, string::String};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataType {
    pub vendor_id: String,
    /// Extra vendor properties preserved on round-trip.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl CustomDataType {
    #[must_use]
    pub fn new(vendor_id: String) -> Self {
        Self {
            vendor_id,
            extra: BTreeMap::new(),
        }
    }
}
