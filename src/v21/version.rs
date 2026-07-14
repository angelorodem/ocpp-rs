//! OCPP 2.0.1 vs 2.1 negotiation helpers.
//!
//! OCPP 2.1 schemas are additive over 2.0.1. Keep a single type tree (`v21`) and
//! gate 2.1-only actions / message types using the negotiated WebSocket subprotocol.

/// Negotiated OCPP-J subprotocol on the WebSocket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NegotiatedVersion {
    /// Subprotocol `ocpp2.0.1`
    Ocpp201,
    /// Subprotocol `ocpp2.1`
    Ocpp21,
}

impl NegotiatedVersion {
    /// Parse a `Sec-WebSocket-Protocol` token.
    #[must_use]
    pub fn from_subprotocol(proto: &str) -> Option<Self> {
        match proto {
            "ocpp2.0.1" => Some(Self::Ocpp201),
            "ocpp2.1" => Some(Self::Ocpp21),
            _ => None,
        }
    }

    /// Wire subprotocol string.
    #[must_use]
    pub const fn as_subprotocol(self) -> &'static str {
        match self {
            Self::Ocpp201 => "ocpp2.0.1",
            Self::Ocpp21 => "ocpp2.1",
        }
    }
}

/// Actions introduced in OCPP 2.1 (not present in 2.0.1).
///
/// Includes DER, tariffs, battery swap, periodic event streams, and related actions.
pub const OCPP21_ONLY_ACTIONS: &[&str] = &[
    "AFRRSignal",
    "AdjustPeriodicEventStream",
    "BatterySwap",
    "ChangeTransactionTariff",
    "ClearDERControl",
    "ClearTariffs",
    "ClosePeriodicEventStream",
    "GetDERControl",
    "GetPeriodicEventStream",
    "GetTariffs",
    "NotifyAllowedEnergyTransfer",
    "NotifyDERAlarm",
    "NotifyDERStartStop",
    "NotifyPeriodicEventStream",
    "NotifyPriorityCharging",
    "NotifySettlement",
    "NotifyWebPaymentStarted",
    "OpenPeriodicEventStream",
    "PullDynamicScheduleUpdate",
    "ReportDERControl",
    "RequestBatterySwap",
    "SetDERControl",
    "SetDefaultTariff",
    "UpdateDynamicSchedule",
    "UsePriorityCharging",
    "VatNumberValidation",
];

/// Whether `action` is defined only in OCPP 2.1.
#[must_use]
pub fn is_ocpp21_only_action(action: &str) -> bool {
    OCPP21_ONLY_ACTIONS.iter().any(|a| *a == action)
}

/// Whether an OCPP-J message type number is allowed for the negotiated version.
///
/// Types 5 (CALLRESULTERROR) and 6 (SEND) are 2.1-only.
#[must_use]
pub const fn allows_message_type(version: NegotiatedVersion, type_id: u8) -> bool {
    match type_id {
        2 | 3 | 4 => true,
        5 | 6 => matches!(version, NegotiatedVersion::Ocpp21),
        _ => false,
    }
}

/// Whether a CALL/SEND action is allowed for the negotiated version.
#[must_use]
pub fn allows_action(version: NegotiatedVersion, action: &str) -> bool {
    match version {
        NegotiatedVersion::Ocpp21 => true,
        NegotiatedVersion::Ocpp201 => !is_ocpp21_only_action(action),
    }
}
