//! Errata-related wire string constants (reason codes, etc.).
//!
//! This module only ships a few stable wire strings that differ from older drafts;
//! full protocol behavioural errata are not modelled here.

/// `SetChargingProfileResponse.statusInfo.reasonCode` when a profile arrives before
/// `NotifyEVChargingNeedsRequest` (prefer `InvalidMessageSeq` over older spellings).
pub const REASON_INVALID_MESSAGE_SEQ: &str = "InvalidMessageSeq";
