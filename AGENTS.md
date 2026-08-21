# AGENTS.md

## Cursor Cloud specific instructions

`ocpp_rs` is a `#![no_std]` + `alloc` Rust library (crate root at `/workspace`) implementing the OCPP 1.6 and 2.1 wire protocols. There are no long-running services, databases, ports, or env vars — end-to-end validation is build + test + running the `example` binary.

Repo layout (three separate Cargo packages, no shared workspace):
- Root crate `ocpp_rs` — the library.
- `example/` — runnable demo binary (`ocpp_rs` via path dependency).
- `fuzz/` — its own separate workspace; optional libFuzzer targets (needs nightly + `cargo-fuzz`).

Toolchain gotcha: this crate is edition 2024 with MSRV 1.85, but the base image may ship an older Rust (e.g. 1.83). The update script installs/defaults to `stable` (>= 1.85) with `rustfmt` + `clippy`; if you hit `edition 2024 is unstable`/MSRV errors, run `rustup default stable`.

Common commands (see `README.md` for the full list):
- Build: `cargo build` (add `--target thumbv7em-none-eabi` for the optional embedded `no_std` check; requires `rustup target add thumbv7em-none-eabi`).
- Test: `cargo test --all` (default features). Optional-feature coverage: `cargo test --all --features datetime_serialize_rfc3339,schema_validate,device_model_catalog`.
- Lint (CI parity): `cargo clippy --all-targets --all-features` and `cargo fmt --all -- --check`.
- Run demo: `cd example && cargo run` (the `example` crate is not part of the root package, so run it from its own directory).

Notes:
- The optional Python `tools/gen_*.py` codegen scripts require schema extracts under `docs/2-1-raw/` that are not present in a normal clone; CI skips them and they are not needed for library dev.
- Fuzzing is optional and not part of standard validation; it needs a nightly toolchain and `cargo-fuzz`.
