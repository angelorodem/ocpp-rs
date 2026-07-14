//! Errata-related wire string constants (reason codes, etc.).
//!
//! Full Part 2 behavioural errata are not implemented here — only stable strings
//! called out in Edition 2 Errata that differ from older drafts.

/// `SetChargingProfileResponse.statusInfo.reasonCode` when a profile arrives before
/// `NotifyEVChargingNeedsRequest` (errata: was `InvalidMessageSequence`).
pub const REASON_INVALID_MESSAGE_SEQ: &str = "InvalidMessageSeq";
