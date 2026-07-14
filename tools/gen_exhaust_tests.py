#!/usr/bin/env python3
"""Generate exhaustive CALL round-trip integration tests from JSON schemas.

After regenerating tests, also refresh fuzz seeds:
  python3 tools/gen_fuzz_corpus.py
"""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_21 = ROOT / "docs" / "2-1-raw" / "schemas"
SCHEMA_16 = ROOT / "docs" / "1-6-raw" / "schemas"
OUT_21 = ROOT / "tests" / "v21_exhaust_roundtrip.rs"
OUT_16 = ROOT / "tests" / "v16_exhaust_roundtrip.rs"


def snake(name: str) -> str:
    s1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


def sample_value(prop: dict, schema: dict, depth: int = 0) -> object:
    if depth > 32:
        return {}
    if "$ref" in prop:
        ref = prop["$ref"].split("/")[-1]
        defn = schema["definitions"][ref]
        return sample_value(defn, schema, depth + 1)
    if "enum" in prop:
        return prop["enum"][0]
    typ = prop.get("type")
    if isinstance(typ, list):
        typ = next((t for t in typ if t != "null"), "string")
    if typ == "string":
        if prop.get("format") == "date-time":
            return "2024-01-01T00:00:00.000Z"
        return "x"
    if typ == "integer":
        mn = prop.get("minimum", 0)
        return int(mn) if mn is not None else 0
    if typ == "number":
        mn = prop.get("minimum", 0)
        return float(mn) if mn is not None else 0.0
    if typ == "boolean":
        return False
    if typ == "array":
        items = prop.get("items") or {}
        min_items = prop.get("minItems", 0)
        if min_items:
            return [sample_value(items, schema, depth + 1) for _ in range(min_items)]
        return []
    if typ == "object" or "properties" in prop:
        obj = {}
        required = set(prop.get("required") or [])
        props = prop.get("properties") or {}
        for k, v in props.items():
            if k == "customData":
                continue
            if k in required:
                obj[k] = sample_value(v, schema, depth + 1)
        return obj
    return {}


def minimal_payload(schema: dict) -> dict:
    root = {
        "type": "object",
        "properties": schema.get("properties") or {},
        "required": schema.get("required") or [],
    }
    return sample_value(root, schema)


def rust_string_escape(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def gen_v21() -> str:
    lines = [
        "//! AUTO-generated exhaustive v21 CALL round-trips. Regenerate: python tools/gen_exhaust_tests.py",
        "use ocpp_rs::v21::parse::{deserialize_to_message, serialize_message, Message};",
        "",
        "fn assert_call_roundtrip(wire: &str) {",
        "    let msg = deserialize_to_message(wire).unwrap_or_else(|e| panic!(\"parse {wire}: {e}\"));",
        "    assert!(matches!(msg, Message::Call(_) | Message::Send(_)), \"expected Call/Send: {msg:?}\");",
        "    let again = serialize_message(&msg).expect(\"serialize\");",
        "    let back = deserialize_to_message(&again).expect(\"reparse\");",
        "    assert_eq!(msg, back);",
        "}",
        "",
    ]
    for path in sorted(SCHEMA_21.glob("*Request.json")):
        action = path.stem[: -len("Request")]
        schema = json.loads(path.read_text())
        payload = minimal_payload(schema)
        payload_json = json.dumps(payload, separators=(",", ":"))
        wire = f'[2,"t","{action}",{payload_json}]'
        fn = snake(action)
        lines.append("#[test]")
        lines.append(f"fn roundtrip_{fn}() {{")
        lines.append(f'    assert_call_roundtrip(r#"{wire}"#);')
        lines.append("}")
        lines.append("")

    # SEND
    send_path = SCHEMA_21 / "NotifyPeriodicEventStream.json"
    if send_path.exists():
        schema = json.loads(send_path.read_text())
        payload = minimal_payload(schema)
        payload_json = json.dumps(payload, separators=(",", ":"))
        wire = f'[6,"t","NotifyPeriodicEventStream",{payload_json}]'
        lines.append("#[test]")
        lines.append("fn roundtrip_notify_periodic_event_stream_send() {")
        lines.append(f'    assert_call_roundtrip(r#"{wire}"#);')
        lines.append("}")
        lines.append("")

    return "\n".join(lines)


def gen_v16() -> str:
    lines = [
        "//! AUTO-generated exhaustive v16 CALL round-trips. Regenerate: python tools/gen_exhaust_tests.py",
        "use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message, Message};",
        "",
        "fn assert_call_roundtrip(wire: &str) {",
        "    let msg = deserialize_to_message(wire).unwrap_or_else(|e| panic!(\"parse {wire}: {e}\"));",
        "    assert!(matches!(msg, Message::Call(_)), \"expected Call: {msg:?}\");",
        "    let again = serialize_message(&msg).expect(\"serialize\");",
        "    let back = deserialize_to_message(&again).expect(\"reparse\");",
        "    assert_eq!(msg, back);",
        "}",
        "",
    ]
    # Core schemas
    for path in sorted(SCHEMA_16.glob("*.json")):
        stem = path.stem
        if stem.endswith("Response"):
            continue
        schema = json.loads(path.read_text())
        payload = minimal_payload(schema)
        payload_json = json.dumps(payload, separators=(",", ":"))
        wire = f'[2,"t","{stem}",{payload_json}]'
        fn = snake(stem)
        lines.append("#[test]")
        lines.append(f"fn roundtrip_{fn}() {{")
        lines.append(f'    assert_call_roundtrip(r#"{wire}"#);')
        lines.append("}")
        lines.append("")

    # Security whitepaper fixtures (not in core 1.6 schema dir)
    security = [
        (
            "GetLog",
            '{"logType":"DiagnosticsLog","requestId":1,"log":{"remoteLocation":"ftp://x"}}',
        ),
        (
            "DeleteCertificate",
            '{"certificateHashData":{"hashAlgorithm":"SHA256","issuerNameHash":"a","issuerKeyHash":"b","serialNumber":"1"}}',
        ),
        (
            "InstallCertificate",
            '{"certificateType":"CentralSystemRootCertificate","certificate":"PEM"}',
        ),
        (
            "GetInstalledCertificateIds",
            '{"certificateType":"ManufacturerRootCertificate"}',
        ),
        (
            "SignedUpdateFirmware",
            '{"requestId":1,"firmware":{"location":"https://x","retrieveDateTime":"2024-01-01T00:00:00.000Z","signingCertificate":"C","signature":"S"}}',
        ),
        (
            "SecurityEventNotification",
            '{"type":"FirmwareUpdated","timestamp":"2024-01-01T00:00:00.000Z"}',
        ),
        (
            "ExtendedTriggerMessage",
            '{"requestedMessage":"BootNotification"}',
        ),
        (
            "CertificateSigned",
            '{"certificateChain":"CERT"}',
        ),
        (
            "SignCertificate",
            '{"csr":"CSR"}',
        ),
        (
            "LogStatusNotification",
            '{"status":"Uploaded","requestId":1}',
        ),
        (
            "SignedFirmwareStatusNotification",
            '{"status":"Downloaded"}',
        ),
    ]
    for action, payload_json in security:
        wire = f'[2,"t","{action}",{payload_json}]'
        fn = snake(action)
        lines.append("#[test]")
        lines.append(f"fn roundtrip_security_{fn}() {{")
        lines.append(f'    assert_call_roundtrip(r#"{wire}"#);')
        lines.append("}")
        lines.append("")

    return "\n".join(lines)


def main() -> None:
    OUT_21.write_text(gen_v21())
    OUT_16.write_text(gen_v16())
    print(f"wrote {OUT_21}")
    print(f"wrote {OUT_16}")


if __name__ == "__main__":
    main()
