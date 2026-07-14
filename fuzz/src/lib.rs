//! Shared helpers for structure-aware and corruption fuzz targets.

use arbitrary::{Arbitrary, Result as ArbitraryResult, Unstructured};
use serde_json::{Map, Number, Value};

/// Well-known OCPP 1.6 CALL actions (core + security whitepaper).
pub const V16_ACTIONS: &[&str] = &[
    "Authorize",
    "BootNotification",
    "CancelReservation",
    "CertificateSigned",
    "ChangeAvailability",
    "ChangeConfiguration",
    "ClearCache",
    "ClearChargingProfile",
    "DataTransfer",
    "DeleteCertificate",
    "DiagnosticsStatusNotification",
    "ExtendedTriggerMessage",
    "FirmwareStatusNotification",
    "GetCompositeSchedule",
    "GetConfiguration",
    "GetDiagnostics",
    "GetInstalledCertificateIds",
    "GetLocalListVersion",
    "GetLog",
    "Heartbeat",
    "InstallCertificate",
    "LogStatusNotification",
    "MeterValues",
    "RemoteStartTransaction",
    "RemoteStopTransaction",
    "ReserveNow",
    "Reset",
    "SecurityEventNotification",
    "SendLocalList",
    "SetChargingProfile",
    "SignCertificate",
    "SignedFirmwareStatusNotification",
    "SignedUpdateFirmware",
    "StartTransaction",
    "StatusNotification",
    "StopTransaction",
    "TriggerMessage",
    "UnlockConnector",
    "UpdateFirmware",
];

/// All OCPP 2.1 CALL actions + SEND NotifyPeriodicEventStream (fuzzer still invents junk).
pub const V21_ACTIONS: &[&str] = &[
    "AFRRSignal",
    "AdjustPeriodicEventStream",
    "Authorize",
    "BatterySwap",
    "BootNotification",
    "CancelReservation",
    "CertificateSigned",
    "ChangeAvailability",
    "ChangeTransactionTariff",
    "ClearCache",
    "ClearChargingProfile",
    "ClearDERControl",
    "ClearDisplayMessage",
    "ClearTariffs",
    "ClearVariableMonitoring",
    "ClearedChargingLimit",
    "ClosePeriodicEventStream",
    "CostUpdated",
    "CustomerInformation",
    "DataTransfer",
    "DeleteCertificate",
    "FirmwareStatusNotification",
    "Get15118EVCertificate",
    "GetBaseReport",
    "GetCertificateChainStatus",
    "GetCertificateStatus",
    "GetChargingProfiles",
    "GetCompositeSchedule",
    "GetDERControl",
    "GetDisplayMessages",
    "GetInstalledCertificateIds",
    "GetLocalListVersion",
    "GetLog",
    "GetMonitoringReport",
    "GetPeriodicEventStream",
    "GetReport",
    "GetTariffs",
    "GetTransactionStatus",
    "GetVariables",
    "Heartbeat",
    "InstallCertificate",
    "LogStatusNotification",
    "MeterValues",
    "NotifyAllowedEnergyTransfer",
    "NotifyChargingLimit",
    "NotifyCustomerInformation",
    "NotifyDERAlarm",
    "NotifyDERStartStop",
    "NotifyDisplayMessages",
    "NotifyEVChargingNeeds",
    "NotifyEVChargingSchedule",
    "NotifyEvent",
    "NotifyMonitoringReport",
    "NotifyPeriodicEventStream",
    "NotifyPriorityCharging",
    "NotifyReport",
    "NotifySettlement",
    "NotifyWebPaymentStarted",
    "OpenPeriodicEventStream",
    "PublishFirmware",
    "PublishFirmwareStatusNotification",
    "PullDynamicScheduleUpdate",
    "ReportChargingProfiles",
    "ReportDERControl",
    "RequestBatterySwap",
    "RequestStartTransaction",
    "RequestStopTransaction",
    "ReservationStatusUpdate",
    "ReserveNow",
    "Reset",
    "SecurityEventNotification",
    "SendLocalList",
    "SetChargingProfile",
    "SetDERControl",
    "SetDefaultTariff",
    "SetDisplayMessage",
    "SetMonitoringBase",
    "SetMonitoringLevel",
    "SetNetworkProfile",
    "SetVariableMonitoring",
    "SetVariables",
    "SignCertificate",
    "StatusNotification",
    "TransactionEvent",
    "TriggerMessage",
    "UnlockConnector",
    "UnpublishFirmware",
    "UpdateDynamicSchedule",
    "UpdateFirmware",
    "UsePriorityCharging",
    "VatNumberValidation",
];

pub const V16_RPC_CODES: &[&str] = &[
    "NotImplemented",
    "NotSupported",
    "InternalError",
    "ProtocolError",
    "SecurityError",
    "FormationViolation",
    "PropertyConstraintViolation",
    "OccurenceConstraintViolation",
    "TypeConstraintViolation",
    "GenericError",
];

pub const V21_RPC_CODES: &[&str] = &[
    "FormatViolation",
    "GenericError",
    "InternalError",
    "MessageTypeNotSupported",
    "NotImplemented",
    "NotSupported",
    "OccurrenceConstraintViolation",
    "PropertyConstraintViolation",
    "ProtocolError",
    "RpcFrameworkError",
    "SecurityError",
    "TypeConstraintViolation",
];

/// Bounded JSON value so we do not explode memory / depth.
#[derive(Debug, Clone)]
pub struct BoundedJson(pub Value);

impl<'a> Arbitrary<'a> for BoundedJson {
    fn arbitrary(u: &mut Unstructured<'a>) -> ArbitraryResult<Self> {
        Ok(Self(arb_value(u, 0)?))
    }
}

fn arb_value(u: &mut Unstructured<'_>, depth: u8) -> ArbitraryResult<Value> {
    if depth > 4 {
        return Ok(Value::Null);
    }
    match u.int_in_range(0..=7)? {
        0 => Ok(Value::Null),
        1 => Ok(Value::Bool(bool::arbitrary(u)?)),
        2 => {
            let n = i32::arbitrary(u)?;
            Ok(Value::Number(Number::from(n)))
        }
        3 => {
            let s = arb_string(u, 64)?;
            Ok(Value::String(s))
        }
        4 => {
            let len = u.int_in_range(0..=4)?;
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len {
                arr.push(arb_value(u, depth + 1)?);
            }
            Ok(Value::Array(arr))
        }
        _ => {
            let len = u.int_in_range(0..=6)?;
            let mut map = Map::new();
            for _ in 0..len {
                let key = arb_json_key(u)?;
                map.insert(key, arb_value(u, depth + 1)?);
            }
            Ok(Value::Object(map))
        }
    }
}

fn arb_string(u: &mut Unstructured<'_>, max: usize) -> ArbitraryResult<String> {
    let kind = u.int_in_range(0..=5)?;
    Ok(match kind {
        0 => String::new(),
        1 => "x".repeat(u.int_in_range(0..=max)?),
        2 => "a".repeat(37), // overlong MessageId-ish
        3 => "2024-01-01T00:00:00.000Z".into(),
        4 => {
            // Control / awkward chars
            let mut s = String::new();
            let n = u.int_in_range(0..=16)?;
            for _ in 0..n {
                s.push(char::from(u.int_in_range(0u8..=0x7f)?));
            }
            s
        }
        _ => {
            let n = u.int_in_range(0..=max)?;
            let bytes = u.bytes(n)?;
            String::from_utf8_lossy(bytes).into_owned()
        }
    })
}

fn arb_json_key(u: &mut Unstructured<'_>) -> ArbitraryResult<String> {
    const KEYS: &[&str] = &[
        "id",
        "status",
        "connectorId",
        "evseId",
        "customData",
        "vendorId",
        "currentTime",
        "interval",
        "reason",
        "chargingStation",
        "model",
        "vendorName",
        "idToken",
        "transactionId",
        "meterValue",
        "type",
        "value",
        "timestamp",
        "errorCode",
        "info",
        "requestId",
        "data",
        "message",
        "payload",
    ];
    if u.ratio(3, 4)? {
        Ok((*u.choose(KEYS)?).into())
    } else {
        arb_string(u, 24)
    }
}

fn pick<'a>(u: &mut Unstructured<'a>, list: &[&'static str]) -> ArbitraryResult<&'static str> {
    Ok(*u.choose(list)?)
}

fn arb_message_id(u: &mut Unstructured<'_>) -> ArbitraryResult<String> {
    match u.int_in_range(0..=4)? {
        0 => Ok("1".into()),
        1 => Ok(uuid_like(u)?),
        2 => Ok("a".repeat(36)),
        3 => Ok("a".repeat(37)), // invalid length
        _ => arb_string(u, 48),
    }
}

fn uuid_like(u: &mut Unstructured<'_>) -> ArbitraryResult<String> {
    Ok(format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u.arbitrary::<u32>()?,
        u.arbitrary::<u16>()?,
        u.arbitrary::<u16>()?,
        u.arbitrary::<u16>()?,
        u.arbitrary::<u64>()? & 0xffff_ffff_ffff,
    ))
}

/// Valid-looking OCPP-J frame (still may fail schema / serde).
#[derive(Debug, Clone)]
pub struct StructuredFrame {
    pub wire: String,
}

#[derive(Debug, Clone, Arbitrary)]
pub enum FrameKind {
    Call,
    CallResult,
    CallError,
    /// 2.1 only
    CallResultError,
    /// 2.1 only
    Send,
    /// Wrong arity / type on purpose
    Weird,
}

impl StructuredFrame {
    pub fn arbitrary_v16(u: &mut Unstructured<'_>) -> ArbitraryResult<Self> {
        Self::arbitrary_version(u, false)
    }

    pub fn arbitrary_v21(u: &mut Unstructured<'_>) -> ArbitraryResult<Self> {
        Self::arbitrary_version(u, true)
    }

    fn arbitrary_version(u: &mut Unstructured<'_>, v21: bool) -> ArbitraryResult<Self> {
        let kind = if v21 {
            FrameKind::arbitrary(u)?
        } else {
            match u.int_in_range(0..=3)? {
                0 => FrameKind::Call,
                1 => FrameKind::CallResult,
                2 => FrameKind::CallError,
                _ => FrameKind::Weird,
            }
        };
        let id = arb_message_id(u)?;
        let actions = if v21 { V21_ACTIONS } else { V16_ACTIONS };
        let rpc = if v21 { V21_RPC_CODES } else { V16_RPC_CODES };
        let payload = BoundedJson::arbitrary(u)?.0;

        let wire = match kind {
            FrameKind::Call | FrameKind::Send => {
                let type_id = if matches!(kind, FrameKind::Send) { 6 } else { 2 };
                let action = if u.ratio(7, 8)? {
                    pick(u, actions)?.to_string()
                } else {
                    arb_string(u, 40)?
                };
                // Prefer object payload; sometimes wrong JSON type
                let body = if u.ratio(6, 7)? {
                    if payload.is_object() {
                        payload
                    } else {
                        Value::Object(Map::new())
                    }
                } else {
                    payload
                };
                serde_json::to_string(&Value::Array(vec![
                    Value::from(type_id),
                    Value::String(id),
                    Value::String(action),
                    body,
                ]))
                .unwrap_or_else(|_| "[]".into())
            }
            FrameKind::CallResult => serde_json::to_string(&Value::Array(vec![
                Value::from(3),
                Value::String(id),
                if u.ratio(5, 6)? {
                    if payload.is_object() {
                        payload
                    } else {
                        Value::Object(Map::new())
                    }
                } else {
                    payload
                },
            ]))
            .unwrap_or_else(|_| "[]".into()),
            FrameKind::CallError | FrameKind::CallResultError => {
                let type_id = if matches!(kind, FrameKind::CallResultError) {
                    5
                } else {
                    4
                };
                let code = if u.ratio(6, 7)? {
                    pick(u, rpc)?.to_string()
                } else {
                    arb_string(u, 32)?
                };
                let desc = arb_string(u, 64)?;
                let details = if u.ratio(4, 5)? {
                    Value::Object(Map::new())
                } else {
                    payload
                };
                serde_json::to_string(&Value::Array(vec![
                    Value::from(type_id),
                    Value::String(id),
                    Value::String(code),
                    Value::String(desc),
                    details,
                ]))
                .unwrap_or_else(|_| "[]".into())
            }
            FrameKind::Weird => {
                // Intentionally broken-but-JSON-looking frames
                match u.int_in_range(0..=5)? {
                    0 => format!(r#"[9, "{id}", {{}}]"#),
                    1 => format!(r#"[2, "{id}"]"#),
                    2 => format!(r#"[2, "{id}", "Heartbeat", null]"#),
                    3 => format!(r#"[2, "{id}", "Heartbeat", []]"#),
                    4 => format!(r#"[3, "{id}", null]"#),
                    _ => format!(r#"[2, "{id}", "Heartbeat", {{"nested":{payload}}}]"#, payload = payload),
                }
            }
        };
        Ok(Self { wire })
    }
}

/// How to corrupt an otherwise valid UTF-8 OCPP-J string.
#[derive(Debug, Clone, Arbitrary)]
pub enum Corruption {
    Truncate { keep_frac: u8 },
    FlipByte { index: u16, xor: u8 },
    InsertByte { index: u16, byte: u8 },
    DeleteByte { index: u16 },
    DuplicatePrefix { times: u8 },
    AppendGarbage { n: u8 },
    SpliceNull,
    SpliceInvalidUtf8,
    WrapInArray,
    DropCommas,
    UppercaseAction,
}

impl Corruption {
    pub fn apply(&self, input: &str) -> Vec<u8> {
        let mut bytes = input.as_bytes().to_vec();
        if bytes.is_empty() {
            return bytes;
        }
        match self {
            Self::Truncate { keep_frac } => {
                let frac = (*keep_frac as usize) % 100;
                let keep = (bytes.len() * frac) / 100;
                bytes.truncate(keep.max(1).min(bytes.len()));
            }
            Self::FlipByte { index, xor } => {
                let i = (*index as usize) % bytes.len();
                bytes[i] ^= if *xor == 0 { 1 } else { *xor };
            }
            Self::InsertByte { index, byte } => {
                let i = (*index as usize) % (bytes.len() + 1);
                bytes.insert(i, *byte);
            }
            Self::DeleteByte { index } => {
                if !bytes.is_empty() {
                    let i = (*index as usize) % bytes.len();
                    bytes.remove(i);
                }
            }
            Self::DuplicatePrefix { times } => {
                let n = (*times as usize % 4) + 1;
                let prefix = bytes.clone();
                for _ in 0..n {
                    bytes.extend_from_slice(&prefix);
                }
            }
            Self::AppendGarbage { n } => {
                for i in 0..(*n as usize % 32) {
                    bytes.push(b'A'.wrapping_add(i as u8));
                }
            }
            Self::SpliceNull => {
                let i = bytes.len() / 2;
                bytes.insert(i, 0);
            }
            Self::SpliceInvalidUtf8 => {
                let i = bytes.len() / 2;
                bytes.insert(i, 0xff);
                bytes.insert(i + 1, 0xfe);
            }
            Self::WrapInArray => {
                let mut wrapped = Vec::with_capacity(bytes.len() + 2);
                wrapped.push(b'[');
                wrapped.extend_from_slice(&bytes);
                wrapped.push(b']');
                bytes = wrapped;
            }
            Self::DropCommas => {
                bytes.retain(|&b| b != b',');
            }
            Self::UppercaseAction => {
                // Best-effort: uppercase ASCII letters between 2nd and 3rd quotes after type
                if let Ok(s) = std::str::from_utf8(&bytes) {
                    if let Some(up) = uppercase_action_field(s) {
                        bytes = up.into_bytes();
                    }
                }
            }
        }
        bytes
    }
}

fn uppercase_action_field(s: &str) -> Option<String> {
    // Very small heuristic for CALL/SEND: [2,"id","Action",{...}]
    let parts: Vec<&str> = s.splitn(4, ',').collect();
    if parts.len() < 3 {
        return None;
    }
    let mut action = parts[2].to_string();
    action = action
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect();
    Some(format!("{},{},{},{}", parts[0], parts[1], action, parts.get(3).unwrap_or(&"")))
}

/// Valid seed templates used as corruption bases (from exhaust / common paths).
pub const V16_SEEDS: &[&str] = &[
    r#"[2,"1","Heartbeat",{}]"#,
    r#"[2,"1","BootNotification",{"chargePointVendor":"x","chargePointModel":"x"}]"#,
    r#"[2,"1","Authorize",{"idTag":"x"}]"#,
    r#"[2,"1","StatusNotification",{"connectorId":0,"errorCode":"NoError","status":"Available"}]"#,
    r#"[2,"1","MeterValues",{"connectorId":0,"meterValue":[]}]"#,
    r#"[2,"1","StartTransaction",{"connectorId":0,"idTag":"x","meterStart":0,"timestamp":"2024-01-01T00:00:00.000Z"}]"#,
    r#"[2,"1","StopTransaction",{"meterStop":0,"timestamp":"2024-01-01T00:00:00.000Z","transactionId":0}]"#,
    r#"[2,"1","SetChargingProfile",{"connectorId":0,"csChargingProfiles":{"chargingProfileId":0,"stackLevel":0,"chargingProfilePurpose":"ChargePointMaxProfile","chargingProfileKind":"Absolute","chargingSchedule":{"chargingRateUnit":"A","chargingSchedulePeriod":[]}}}]"#,
    r#"[2,"1","SecurityEventNotification",{"type":"FirmwareUpdated","timestamp":"2024-01-01T00:00:00.000Z"}]"#,
    r#"[2,"1","GetLog",{"logType":"DiagnosticsLog","requestId":1,"log":{"remoteLocation":"ftp://x"}}]"#,
    r#"[3,"1",{"currentTime":"2024-01-01T00:00:00.000Z"}]"#,
    r#"[4,"1","GenericError","x",{}]"#,
    r#"[4,"1","FormationViolation","x",{"nested":{"code":42}}]"#,
];

pub const V21_SEEDS: &[&str] = &[
    r#"[2,"1","Heartbeat",{}]"#,
    r#"[2,"1","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"M","vendorName":"V"}}]"#,
    r#"[2,"1","Authorize",{"idToken":{"idToken":"x","type":"x"}}]"#,
    r#"[2,"1","StatusNotification",{"timestamp":"2024-01-01T00:00:00.000Z","connectorStatus":"Available","evseId":0,"connectorId":0}]"#,
    r#"[2,"1","TransactionEvent",{"eventType":"Ended","timestamp":"2024-01-01T00:00:00.000Z","triggerReason":"AbnormalCondition","seqNo":0,"transactionInfo":{"transactionId":"x"}}]"#,
    r#"[2,"1","GetVariables",{"getVariableData":[{"component":{"name":"x"},"variable":{"name":"x"}}]}]"#,
    r#"[2,"1","SetVariables",{"setVariableData":[{"attributeValue":"x","component":{"name":"x"},"variable":{"name":"x"}}]}]"#,
    r#"[2,"1","NotifyEvent",{"generatedAt":"2024-01-01T00:00:00.000Z","seqNo":0,"eventData":[{"eventId":0,"timestamp":"2024-01-01T00:00:00.000Z","trigger":"Alerting","actualValue":"x","component":{"name":"x"},"eventNotificationType":"HardWiredNotification","variable":{"name":"x"}}]}]"#,
    r#"[2,"1","RequestStartTransaction",{"idToken":{"idToken":"x","type":"x"},"remoteStartId":0}]"#,
    r#"[2,"1","MeterValues",{"evseId":0,"meterValue":[{"sampledValue":[{"value":0.0}],"timestamp":"2024-01-01T00:00:00.000Z"}]}]"#,
    r#"[2,"1","SetDERControl",{"isDefault":false,"controlId":"x","controlType":"EnterService"}]"#,
    r#"[2,"1","GetTariffs",{"evseId":0}]"#,
    r#"[6,"1","NotifyPeriodicEventStream",{"data":[{"t":0.0,"v":"x"}],"id":0,"pending":0,"basetime":"2024-01-01T00:00:00.000Z"}]"#,
    r#"[3,"1",{"currentTime":"2024-01-01T00:00:00.000Z"}]"#,
    r#"[4,"1","NotSupported","x",{}]"#,
    r#"[4,"1","FormatViolation","x",{"nested":{"code":42}}]"#,
    r#"[5,"1","GenericError","x",{}]"#,
];

#[derive(Debug)]
pub struct CorruptInput {
    pub bytes: Vec<u8>,
}

impl CorruptInput {
    pub fn arbitrary_v16(u: &mut Unstructured<'_>) -> ArbitraryResult<Self> {
        Self::arbitrary_version(u, false)
    }

    pub fn arbitrary_v21(u: &mut Unstructured<'_>) -> ArbitraryResult<Self> {
        Self::arbitrary_version(u, true)
    }

    fn arbitrary_version(u: &mut Unstructured<'_>, v21: bool) -> ArbitraryResult<Self> {
        // Prefer mutating a real seed; sometimes start from a structured frame.
        let base = if u.ratio(3, 4)? {
            let seeds = if v21 { V21_SEEDS } else { V16_SEEDS };
            (*u.choose(seeds)?).to_string()
        } else if v21 {
            StructuredFrame::arbitrary_v21(u)?.wire
        } else {
            StructuredFrame::arbitrary_v16(u)?.wire
        };

        let n = u.int_in_range(1..=4)?;
        let mut bytes = base.into_bytes();
        for _ in 0..n {
            let corruption = Corruption::arbitrary(u)?;
            if let Ok(s) = std::str::from_utf8(&bytes) {
                bytes = corruption.apply(s);
            } else {
                let flip = Corruption::FlipByte {
                    index: u.arbitrary()?,
                    xor: u.arbitrary()?,
                };
                bytes = flip.apply(&String::from_utf8_lossy(&bytes));
            }
        }
        Ok(Self { bytes })
    }
}
