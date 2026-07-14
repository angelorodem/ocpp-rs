# OCPP 2.0.1 compatibility

OCPP **2.1** is an **additive** extension of **2.0.1**: shared messages keep the same names and
required fields; 2.1 adds optional fields and new actions. This crate exposes a single
[`ocpp_rs::v21`](../src/v21.rs) type tree.

## Subprotocol negotiation

Offer / select WebSocket subprotocols in preference order, for example:

```text
Sec-WebSocket-Protocol: ocpp2.1, ocpp2.0.1, ocpp1.6
```

| Token | Module | Notes |
|-------|--------|--------|
| `ocpp1.6` | [`v16`](../src/v16.rs) | Types 2–4 only |
| `ocpp2.0.1` | `v21` + [`NegotiatedVersion::Ocpp201`](../src/v21/version.rs) | No CALLRESULTERROR(5) / SEND(6); reject 2.1-only actions |
| `ocpp2.1` | `v21` + `NegotiatedVersion::Ocpp21` | Full framing |

```rust
use ocpp_rs::v21::version::{allows_action, allows_message_type, NegotiatedVersion};

let v = NegotiatedVersion::from_subprotocol("ocpp2.0.1").unwrap();
assert!(!allows_message_type(v, 6));
assert!(!allows_action(v, "SetDERControl"));
assert!(allows_action(v, "BootNotification"));
```

## Known 2.0.1 ↔ 2.1 exceptions

- `SignCertificateRequest.requestId` (optional in 2.1): when present, CSMS must echo it on `CertificateSigned`.
- `NotifyMonitoringReport` / `VariableMonitoringType.eventNotificationType` became required in 2.1; a 2.0.1 CSMS that ignores the field still works.

## Helpers

See [`v21::version`](../src/v21/version.rs):

- `is_ocpp21_only_action`
- `allows_message_type`
- `allows_action`
- `OCPP21_ONLY_ACTIONS`
