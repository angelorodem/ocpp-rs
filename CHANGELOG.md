# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-07-14

Breaking release for OCPP 1.6 CallResult handling, shared datetime helpers, and several wire-type fixes.
Full guide: [`guides/migration-0.3.md`](guides/migration-0.3.md).

### Added

- OCPP **2.1** module (`v21`) with CallResult correlation (`PendingCalls`, action-name resolve, probe last resort)
- OCPP **1.6** CallResult correlation aligned with 2.1 (`CallResultRaw`, `TypedCallResult`, `PendingCalls` / `PendingActionNames`)
- Shared [`datetime`](src/datetime.rs) helpers: RFC3339 **parse** always; serialize defaults to `%.3fZ`
- Cargo feature `datetime_serialize_rfc3339` for RFC3339 millis emit
- Per-message 1.6 status enums (replacing `ParsedGenericStatus`)
- `Call::action_kind()` on v16 (parity with v21)
- Tracked guides under [`guides/`](guides/)
- CI: OS/toolchain matrix, datetime feature tests, `no_std` + `thumbv7em-none-eabi` build, MSRV 1.85

### Changed

- `Response::get_response` returns `Result<Message>` (v16 and v21); no silent JSON `null`
- v16 `CallError.error_details` is `BTreeMap<String, Value>`
- Authorize.conf requires `idTagInfo`; StopTransaction.conf keeps optional `idTagInfo`
- Nested charging / auth-list / security fields use real structs instead of `BTreeMap<String, String>`
- Chrono dependency kept `alloc`+`serde` only (no `std`/`clock` feature unification from dev-deps)
- `rust-version = "1.85"` (edition 2024)

### Removed

- Untagged CallResult guessing (`ResultPayload`, `EmptyResponses`, `StatusResponses`, …)
- `GenericIdTagInfo`, `ParsedGenericStatus`, `GenericStatusResponse`
- `datetime_parse_rfc3339` feature (parse is always RFC3339)

### Fixed

- `UnitOfMeasure` accepts legacy `Celcius` spelling
- Spec-aligned enum `Default`s (`Wh`, `Energy.Active.Import.Register`, `Outlet`, `Sample.Periodic`)
- Security Whitepaper firmware / trigger enum variants; optional `requestId` on signed firmware status

## [0.2.x]

Prior OCPP 1.6-focused releases (untagged CallResult payload enums).
