# OCPP-RS

OCPP-RS is a Rust library for implementing the Open Charge Point Protocol (OCPP) in Rust.

It supports **OCPP 1.6** and **OCPP 2.1** (2.0.1-compatible additive schemas).

[Documentation](https://docs.rs/ocpp_rs/latest/ocpp_rs/)

- Full implementation of OCPP 1.6 and OCPP 2.1 message payloads
- Batteries included: OCPP-J parse/serialize for both versions
- CallResult typing via `PendingCalls` / action-name correlation (no untagged guessing)
- **`#![no_std]` + `alloc`** — library code never uses `std` (baremetal-friendly). You must provide a global allocator (e.g. `embedded-alloc`, `linked_list_allocator`, or your RTOS heap).
- Inspired by a [python ocpp library](https://github.com/mobilityhouse/ocpp); 2.1 layout inspired by [rust-ocpp](https://github.com/tommymalmqvist/rust-ocpp)

## `no_std` / firmware

The published library (`src/`) is `#![no_std]` + `alloc` end-to-end: zero `std::` usage, and dependencies are built with `default-features = false` (chrono without `clock`/`std`). Integration tests and `example/` may use `std` for host runners only — that does not leak into firmware consumers.

You must provide a **global allocator** on baremetal (e.g. `embedded-alloc`, `linked_list_allocator`, or your RTOS heap).

```toml
# Typical embedded consumer
[dependencies]
ocpp_rs = { version = "0.3", default-features = false }
# plus your allocator crate / #[global_allocator]
```

Sanity checks:

```bash
# Library must build under #![no_std] (host is enough to catch std usage in src/)
cargo build --lib

# Optional: freestanding target (requires rustup target)
cargo build --lib --target thumbv7em-none-eabi

# Chrono must not enable std/clock via feature unification
cargo tree -p chrono -f '{p} {f}' | head -5
# expect: chrono … alloc,serde   (not std/clock)
```

## Usage

```toml
[dependencies]
ocpp-rs = "^0.3"
```

Upgrading from 0.2.x: see [guides/migration-0.3.md](guides/migration-0.3.md).

# Particularities

## OCPP 1.6 CallResult

CALLRESULT has no action on the wire. Blind parse yields `CallResultRaw`. Correlate with `PendingCalls` (same idea as 2.1):

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

- **Sticky sessions** or **Redis `messageId → action name`** + `resolve_with_action_name` for load-balanced deploys
- **Datetime**: always parse RFC3339; serialize defaults to `%.3fZ` (optional `datetime_serialize_rfc3339`) — [guides/datetime-features.md](guides/datetime-features.md)
- Last resort: `try_resolve_unique` / `probe_candidates` (often ambiguous for `{}`)

Details: [`v16::pending`](src/v16/pending.rs), [migration guide](guides/migration-0.3.md).

## OCPP 2.1

Same CallResult model under `v21::pending`. Framing includes types 2–6 (CALLRESULTERROR, SEND). See README sections in crate docs / `v21` module.

### Load-balanced example (2.1)

```rust
use ocpp_rs::v21::pending::resolve_with_action_name;
use ocpp_rs::v21::parse::{deserialize_to_message, Message};

let Message::CallResult(raw) = deserialize_to_message(wire)? else { /* … */ };
let typed = resolve_with_action_name(raw, &redis_getdel(&raw.unique_id)?)?;
```

## Example (1.6 CALL)

```rust
use ocpp_rs::v16::parse::{self, Message};
use ocpp_rs::v16::call::Action;

let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
let incoming_message = parse::deserialize_to_message(incoming_text);
if let Ok(Message::Call(call)) = incoming_message {
    match call.payload {
        Action::BootNotification(_boot_notification) => { /* … */ }
        _ => {}
    }
}
```

Sending a CALLRESULT:

```rust
use ocpp_rs::v16::call_result::CallResultRaw;
use ocpp_rs::v16::parse::{self, Message};

let response = Message::CallResult(CallResultRaw::new(
    "1234".into(),
    serde_json::to_value(/* your *Response struct */)?,
));
let json = parse::serialize_message(&response)?;
```

## Contributing

Contributions are welcome. Please add/update tests with behavior changes.

## Roadmap

- Harden OCPP 2.1 field coverage against Part 3 schemas / errata
- Fuzzing for v16 and v21 wire parsers
