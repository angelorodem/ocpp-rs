# Migrating to ocpp-rs 0.3.0

Crate version **0.3.0** is a breaking release for the whole package (OCPP 1.6 and 2.1 share framing helpers).
If you only used 1.6 CallResult guessing, the CallResult sections below are the main work.
If you already used 2.1, still update `Response::get_response` and any `CallError` detail maps.

Study materials under `docs/` (schemas, errata notes) are local/dev extracts and may not be in git.
These guides under `guides/` are meant to ship with the repo.

---

## Dependency

```toml
# before
ocpp-rs = "^0.2"

# after
ocpp-rs = "^0.3"

# Optional: emit RFC3339 millis instead of %.3fZ (parse is always RFC3339)
# ocpp-rs = { version = "^0.3", features = ["datetime_serialize_rfc3339"] }
```

Requires a Rust toolchain that supports **edition 2024** (typically **1.85+**). The library is `#![no_std]` + `alloc` (global allocator required on baremetal).

---

## 1. Blind CALLRESULT parse (1.6)

OCPP 1.6 `CALLRESULT` is `[3, messageId, payload]` with **no action name**.

**Before (0.2.x)** — untagged guessing:

```rust
match deserialize_to_message(data)? {
    Message::CallResult(CallResult { payload: ResultPayload::…, .. }) => { /* … */ }
    // …
}
```

**After (0.3)** — always raw until you correlate:

```rust
use ocpp_rs::v16::call_result::CallResultRaw;
use ocpp_rs::v16::parse::{deserialize_to_message, Message};

match deserialize_to_message(data)? {
    Message::CallResult(CallResultRaw { unique_id, payload, .. }) => {
        // payload is serde_json::Value until you resolve it
    }
    // …
}
```

Same model under [`v21::pending`](../src/v21/pending.rs) for OCPP 2.1.

---

## 2. Receiving responses to calls you sent

**Before** — match `PossibleEmptyResponse` / `PossibleStatusResponse` and reconcile `unique_id` by hand.

**After** — register outbound CALLs, then resolve:

```rust
use ocpp_rs::v16::call::{Action, Call, Heartbeat};
use ocpp_rs::v16::pending::PendingCalls;
use ocpp_rs::v16::parse::TypedMessage;
use ocpp_rs::v16::typed_call_result::TypedCallResult;

let mut pending = PendingCalls::new();
let wire = pending.send_call(Call::new("1".into(), Action::Heartbeat(Heartbeat {})))?;

match pending.deserialize_typed(r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#)? {
    TypedMessage::CallResult(TypedCallResult::Heartbeat(cr)) => { /* cr.payload */ }
    other => { /* Call / CallError */ }
}
```

### Already parsed `Message`

```rust
let typed = pending.message_to_typed(message)?;
```

### Load-balanced / shared store

Persist the action **name** string, then on any node:

| Version | How to get the name on send |
|---------|-----------------------------|
| 1.6 | `call.payload.as_ref()` (`Action` is `AsRefStr`) → e.g. `"Heartbeat"` |
| 2.1 | `call.payload.action_name()` or `call.action_kind()` |

```rust
use ocpp_rs::v16::pending::resolve_with_action_name;

// redis.SET(message_id, "Heartbeat") on send
// let name = redis.GETDEL(message_id) on result
let typed = resolve_with_action_name(raw, &name)?;
```

In-process string map: [`PendingActionNames`](../src/v16/pending.rs) (and the v21 twin).

### New errors

| Error | When |
|-------|------|
| `UnknownPendingMessageId` | CALLRESULT `unique_id` was never registered |
| `UnknownActionName` | Action string is not a known CALL action (1.6 or 2.1) |
| `AmbiguousCallResult` | Probe matched multiple schemas (typical for `{}` / status-only) |

---

## 3. Known response type

```rust
use ocpp_rs::v16::call_result::{deserialize_call_result, BootNotification};

let result = deserialize_call_result::<BootNotification>(wire)?;
// or: raw.into_typed::<BootNotification>()?
```

Serialize a typed result: `parse::serialize_typed_call_result(&typed)?` (v16 and v21).

---

## 4. Building responses

**Before**

```rust
Message::CallResult(CallResult::new(
    id,
    ResultPayload::PossibleStatusResponse(…),
))
```

**After** (preferred) — **returns `Result`** (v16 and v21). Failed serde no longer becomes JSON `null`.

```rust
use ocpp_rs::v16::response_trait::Response;

let msg = request.get_response(unique_id, payload)?;
```

Or build the wire envelope yourself:

```rust
use ocpp_rs::v16::call_result::CallResultRaw;

Message::CallResult(CallResultRaw::new(
    unique_id,
    serde_json::to_value(payload)?,
))
```

---

## 5. Wire / status type changes (1.6)

| Removed / old | Replacement |
|---------------|-------------|
| `ResultPayload` | `CallResultRaw` + `TypedCallResult` |
| `EmptyResponses` / `StatusResponses` | Pending resolve → concrete types |
| `PossibleEmpty` / `Status`, `is_empty()`, `is_only_status()` | Correlation picks the type |
| `ParsedGenericStatus` / `GenericStatusResponse` | Per-message status enums + typed `.conf` (e.g. `UnlockStatus`, `call_result::UnlockConnector`) |
| `GenericIdTagInfo` | `call_result::Authorize { id_tag_info }` (**required**) / `call_result::StopTransaction { id_tag_info: Option<_> }` |
| `SendLocalList.local_authorization_list: Vec<String>` | `Option<Vec<AuthorizationData>>` |
| `BTreeMap` placeholders for profiles / schedules / security nests | `ChargingProfile`, `ChargingSchedule`, `CertificateHashData`, `LogParameters`, `Firmware` |
| `CallError.error_details: BTreeMap<String, String>` | `BTreeMap<String, serde_json::Value>` (v16; v21 already used `Value`) |
| Nested `UnlockConnector { status: GenericStatusResponse }` | Flat `UnlockConnector { status: UnlockStatus }` |

**Authorize.conf** rejects `{}` (missing `idTagInfo`). **StopTransaction.conf** still allows `{}`.

Security Whitepaper types are best-effort (no core 1.6 schemas in-tree); field shapes may need tweaks per whitepaper edition. See comments on [`CertificateHashData`](../src/v16/data_types.rs).

---

## 6. Datetime (v16 and v21)

**Parse** always accepts RFC3339 (including `%.3fZ`, offsets, variable fractional seconds).

**Serialize** defaults to strict `%Y-%m-%dT%H:%M:%S%.3fZ`. Enable `datetime_serialize_rfc3339` to emit chrono RFC3339 millis instead.

In-memory values are always `DateTimeWrapper` / `chrono::DateTime<Utc>`. See [datetime-features.md](datetime-features.md).

---

## 7. Unchanged

- CALL / CALLERROR (and v21 CALLRESULTERROR / SEND) framing shapes
- Most Action request field layouts (aside from nested-type fixes above)
- `no_std` + `alloc`
- CallResult correlation model relative to 0.3’s first cut (raw + pending)
- Default **serialize** form `%.3fZ` (parse is always RFC3339 in 0.3)

---

## 8. Last resort when type is unknown

```rust
use ocpp_rs::v16::pending::try_resolve_unique;

let typed = try_resolve_unique(&raw)?;
// or inspect: raw.probe_candidates()
```

`{}` and `{"status":…}` often match **many** schemas → `AmbiguousCallResult`. Prefer persisting the action name.

---

## Logging

`MessageLogLine::from_message` labels unresolved CALLRESULT as `payload_type: "CallResultRaw"` until you resolve via pending / action name.

---

## See also

- [datetime-features.md](datetime-features.md)
- [`src/v16/pending.rs`](../src/v16/pending.rs) — primary 1.6 design notes
- [`src/v21/pending.rs`](../src/v21/pending.rs) — same pattern for 2.1
- Crate docs: `ocpp_rs` / `datetime`
