# OCPP-RS

Rust protocol library for the [Open Charge Point Protocol](https://www.openchargealliance.org/) (OCPP).

Supports **OCPP 1.6** (JSON) and **OCPP 2.1** (additive / 2.0.1-compatible schemas).

[Documentation](https://docs.rs/ocpp_rs/latest/ocpp_rs/) · [Changelog](CHANGELOG.md) · [Guides](guides/)

## Features

- Full OCPP 1.6 and 2.1 message payloads + OCPP-J parse/serialize
- CallResult typing via `PendingCalls` / action-name correlation
- Typed RPC framework error codes
- Optional payload length/bounds checks (`schema_validate`) and device-model catalogs
- **`#![no_std]` + `alloc`** — zero `std` in library code (global allocator required on baremetal)

## Install

```toml
[dependencies]
ocpp-rs = "0.4"

# Optional:
# features = ["schema_validate", "device_model_catalog", "datetime_serialize_rfc3339"]
```

| Feature | Purpose |
|---------|---------|
| `schema_validate` | Enforce string/array/numeric bounds on CALL (and 2.1 SEND) payloads after parse |
| `device_model_catalog` | Standard 2.1 component/variable name tables |
| `datetime_serialize_rfc3339` | Emit RFC3339 millis instead of `%.3fZ` |

**MSRV:** 1.85 (edition 2024).

Upgrading from **0.2.x:** [guides/migration-0.4.md](guides/migration-0.4.md).

## `no_std` / firmware

```toml
[dependencies]
ocpp_rs = { version = "0.4", default-features = false }
# plus your allocator / #[global_allocator]
```

```bash
cargo build --lib
cargo build --lib --target thumbv7em-none-eabi   # optional freestanding check
cargo tree -p chrono -f '{p} {f}' | head -5       # expect: alloc,serde (not std/clock)
```

## OCPP 1.6 CallResult

CALLRESULT has no action on the wire. Blind parse yields `CallResultRaw`. Correlate with `PendingCalls`:

```rust
use ocpp_rs::v16::call::{Action, Call, Heartbeat};
use ocpp_rs::v16::parse::TypedMessage;
use ocpp_rs::v16::pending::PendingCalls;
use ocpp_rs::v16::typed_call_result::TypedCallResult;

let mut pending = PendingCalls::new();
let _wire = pending.send_call(Call::new("1".into(), Action::Heartbeat(Heartbeat {})))?;
let typed = pending.deserialize_typed(r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#)?;
assert!(matches!(typed, TypedMessage::CallResult(TypedCallResult::Heartbeat(_))));
```

- Sticky sessions or Redis `messageId → action name` + `resolve_with_action_name`
- Datetime: RFC3339 parse; serialize defaults to `%.3fZ` — [guides/datetime-features.md](guides/datetime-features.md)
- Details: [`v16::pending`](src/v16/pending.rs), [migration guide](guides/migration-0.4.md)

## OCPP 2.1

Same correlation model under `v21::pending`. Framing types **2–6** (CALL, CALLRESULT, CALLERROR, CALLRESULTERROR, SEND).

```rust
use ocpp_rs::v21::call::{Action, Call};
use ocpp_rs::v21::messages::heartbeat::HeartbeatRequest;
use ocpp_rs::v21::parse::TypedMessage;
use ocpp_rs::v21::pending::PendingCalls;
use ocpp_rs::v21::typed_call_result::TypedCallResult;

let mut pending = PendingCalls::new();
let call = Call::new(
    "1".into(),
    Action::Heartbeat(HeartbeatRequest { custom_data: None }),
);
assert_eq!(call.action_kind(), "Heartbeat");
let _wire = pending.send_call(call)?;
let typed = pending.deserialize_typed(
    r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#,
)?;
assert!(matches!(
    typed,
    TypedMessage::CallResult(TypedCallResult::Heartbeat(_))
));
```

2.0.1 subprotocol gating: [guides/ocpp-2.0.1.md](guides/ocpp-2.0.1.md).

### Load-balanced resolve

```rust
use ocpp_rs::v21::pending::resolve_with_action_name;
use ocpp_rs::v21::parse::{deserialize_to_message, Message};

let Message::CallResult(raw) = deserialize_to_message(wire)? else { /* … */ };
let typed = resolve_with_action_name(raw, &redis_getdel(&raw.unique_id)?)?;
```

## Example crate

```bash
cd example && cargo run
```

Covers 1.6 and 2.1: `PendingCalls` correlation, CALL / CALLRESULT / CALLERROR, and 2.0.1 subprotocol gating.

## Example (1.6 CALL)

```rust
use ocpp_rs::v16::parse::{self, Message};
use ocpp_rs::v16::call::Action;

let incoming_text = r#"[2, "19223201", "BootNotification", {"chargePointVendor":"VendorX","chargePointModel":"SingleSocketCharger"}]"#;
let incoming_message = parse::deserialize_to_message(incoming_text)?;
if let Message::Call(call) = incoming_message {
    match call.payload {
        Action::BootNotification(_boot) => { /* … */ }
        _ => {}
    }
}
```

## Out of scope

This is a **protocol wire crate**, not a full stack:

- No SOAP / HTTP binding for 1.6
- No WebSocket, TLS, or Security Profile handshake
- No Charge Point / CSMS state machines or certification test runners

## Contributing

Add or update tests with behavior changes.

### Fuzzing

Requires nightly + [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz):

```bash
rustup toolchain install nightly -c rust-src
cargo +nightly install cargo-fuzz
```

| Target | What it feeds the parser |
|--------|---------------------------|
| `v16_deserialize` / `v21_deserialize` | Raw bytes as UTF-8 |
| `v16_roundtrip` / `v21_roundtrip` | Raw bytes; if parse OK, serialize → reparse |
| `v16_structured` / `v21_structured` | Valid-looking OCPP-J frames (real actions, RPC codes, bounded JSON) |
| `v16_corrupt` / `v21_corrupt` | Seed / structured frames + truncations, bit flips, bad UTF-8, wrong arity, … |

```bash
./fuzz/run_all.sh              # all v16 + v21 targets (300s, 4 workers each)
./fuzz/run_v16.sh              # OCPP 1.6 only
./fuzz/run_v21.sh              # OCPP 2.1 only
./fuzz/run_v21.sh 600 8        # longer / more workers

./fuzz/clean.sh                # artifacts, logs, hash corpus, fuzz/target (keeps *.json seeds)
./fuzz/clean.sh --keep-corpus  # leave discovered hash inputs
./fuzz/clean.sh --keep-target  # leave fuzz/target build cache
```

Or run a single target manually:

```bash
# One target, 5 minutes, 8 workers
cargo +nightly fuzz run v21_corrupt --fuzz-dir fuzz -- -max_total_time=300 -jobs=8 -workers=8
```

Discovered corpus inputs under `fuzz/corpus/` are gitignored (seed `*.json` only).
Regenerate action seeds from exhaust tests: `python3 tools/gen_fuzz_corpus.py`.

License: MIT.
