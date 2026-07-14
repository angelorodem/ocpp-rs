# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-07-14

Breaking release from the **0.2.x** line: OCPP 2.1, CallResult correlation, wire-type fixes,
typed RPC errors, and optional schema / device-model features.
Migration: [`guides/migration-0.4.md`](guides/migration-0.4.md).

### Added

- OCPP **2.1** (`v21`): framing types 2–6, 90 CALL actions, SEND, CallResult correlation
- OCPP **1.6** CallResult correlation (`CallResultRaw`, `TypedCallResult`, `PendingCalls` / `PendingActionNames`)
- Shared [`datetime`](src/datetime.rs): RFC3339 **parse** always; serialize defaults to `%.3fZ`
- Feature `datetime_serialize_rfc3339`
- Feature **`schema_validate`**: Part 3 / 1.6 constraint checks
- Feature **`device_model_catalog`**: `v21::device_model` component/variable catalogs
- Typed RPC error codes (`v16` / `v21` `RpcErrorCode`)
- `Call::action_kind()` on v16 (parity with v21)
- `v21::version` helpers for OCPP 2.0.1 vs 2.1 subprotocol gating
- Per-message 1.6 status enums; nested charging / security wire types
- Always-on MessageId / UniqueId max length 36
- Exhaustive CALL round-trip tests; `fuzz/` targets with seed corpus
- Guides under [`guides/`](guides/); CI matrix (OS, MSRV 1.85, features, `no_std` / thumb)

### Changed

- `Response::get_response` returns `Result<Message>` (v16 and v21)
- `CallError` / `CallResultError` `error_code` is `RpcErrorCode` (was `String`)
- v16 `CallError.error_details` is `BTreeMap<String, Value>`
- `CustomDataType` preserves vendor extras via `extra: BTreeMap<String, Value>`
- Authorize.conf requires `idTagInfo`; StopTransaction.conf keeps optional `idTagInfo`
- Chrono kept `alloc`+`serde` only (no `std`/`clock` from feature unification)
- `rust-version = "1.85"` (edition 2024)
- `OCSPRequestDataType.responder_url` wire name `responderURL`

### Removed

- Untagged CallResult guessing (`ResultPayload`, `EmptyResponses`, `StatusResponses`, …)
- `GenericIdTagInfo`, `ParsedGenericStatus`, `GenericStatusResponse`
- `datetime_parse_rfc3339` (parse is always RFC3339)

### Fixed

- `UnitOfMeasure` accepts legacy `Celcius` spelling
- Spec-aligned enum `Default`s (`Wh`, `Energy.Active.Import.Register`, `Outlet`, `Sample.Periodic`)
- Security Whitepaper firmware / trigger variants; optional `requestId` on signed firmware status
- Errata reason-code constant `InvalidMessageSeq` (`v21::errata`)

## [0.2.x]

Prior OCPP 1.6-focused releases (untagged CallResult payload enums).
