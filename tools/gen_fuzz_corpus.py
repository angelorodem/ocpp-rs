#!/usr/bin/env python3
"""Generate fuzz seed JSON from exhaustive CALL round-trip tests.

Writes identical seeds into:
  fuzz/corpus/v{16,21}_{deserialize,roundtrip}/

Keeps *.json seeds only (see fuzz/clean.sh). Structured/corrupt targets do not
consume OCPP JSON corpora — they use V*_ACTIONS / V*_SEEDS in fuzz/src/lib.rs.

Usage:
  python3 tools/gen_fuzz_corpus.py
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

WIRE_RE = re.compile(
    r'assert_call_roundtrip\(\s*r#"(.*?)"#\s*,?\s*\)',
    re.DOTALL,
)


def camel_to_snake(name: str) -> str:
    out: list[str] = []
    for i, ch in enumerate(name):
        if ch.isupper() and i > 0 and (not name[i - 1].isupper() or (i + 1 < len(name) and name[i + 1].islower())):
            out.append("_")
        out.append(ch.lower())
    return "".join(out)


def extract_wires(path: Path) -> list[tuple[str, str]]:
    """Return (filename_stem, wire_json) pairs."""
    text = path.read_text()
    pairs: list[tuple[str, str]] = []
    seen: set[str] = set()
    for m in WIRE_RE.finditer(text):
        wire = m.group(1)
        # Expect [type,"id","Action",...]
        action_m = re.match(r'\[(\d+),"[^"]*","([^"]+)"', wire)
        if not action_m:
            print(f"warn: could not parse action from {wire[:80]!r}", file=sys.stderr)
            continue
        type_id, action = action_m.group(1), action_m.group(2)
        stem = camel_to_snake(action)
        if type_id == "6":
            stem = f"send_{stem}"
        if stem in seen:
            print(f"warn: duplicate stem {stem}", file=sys.stderr)
            continue
        seen.add(stem)
        pairs.append((stem, wire))
    return pairs


def write_seeds(version: str, call_wires: list[tuple[str, str]], extras: dict[str, str]) -> None:
    targets = [
        ROOT / "fuzz" / "corpus" / f"{version}_deserialize",
        ROOT / "fuzz" / "corpus" / f"{version}_roundtrip",
    ]
    for target in targets:
        target.mkdir(parents=True, exist_ok=True)
        # Remove old seeds so deleted actions / renamed files do not linger.
        for old in target.glob("*.json"):
            old.unlink()
        for stem, wire in call_wires:
            (target / f"{stem}.json").write_text(wire + "\n")
        for stem, wire in extras.items():
            (target / f"{stem}.json").write_text(wire + "\n")


def main() -> None:
    v16 = extract_wires(ROOT / "tests" / "v16_exhaust_roundtrip.rs")
    v21 = extract_wires(ROOT / "tests" / "v21_exhaust_roundtrip.rs")

    v16_extras = {
        "call_result_heartbeat": '[3,"1",{"currentTime":"2024-01-01T00:00:00.000Z"}]',
        "call_error_generic": '[4,"1","GenericError","x",{}]',
        "call_error_formation_violation": '[4,"1","FormationViolation","x",{}]',
        "call_error_occurence_constraint": '[4,"1","OccurenceConstraintViolation","x",{}]',
        "call_error_not_implemented": '[4,"1","NotImplemented","x",{}]',
        "call_error_not_supported": '[4,"1","NotSupported","x",{}]',
        "call_error_internal": '[4,"1","InternalError","x",{}]',
        "call_error_protocol": '[4,"1","ProtocolError","x",{}]',
        "call_error_security": '[4,"1","SecurityError","x",{}]',
        "call_error_property_constraint": '[4,"1","PropertyConstraintViolation","x",{}]',
        "call_error_type_constraint": '[4,"1","TypeConstraintViolation","x",{}]',
        "call_error_unknown_code": '[4,"1","TotallyUnknownCode","x",{}]',
        "call_error_nested_details": '[4,"1","ProtocolError","x",{"nested":{"code":42}}]',
        "message_id_ok_36": '[2,"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","Heartbeat",{}]',
        # Negative / edge — still useful for deserialize; roundtrip no-ops if parse fails.
        "message_id_too_long": '[2,"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","Heartbeat",{}]',
        "call_null_payload": '[2,"1","Heartbeat",null]',
        "call_array_payload": '[2,"1","Heartbeat",[]]',
        "unknown_action": '[2,"1","NotARealAction",{}]',
        "type5_rejected": '[5,"1","GenericError","x",{}]',
        "wrong_arity": '[2,"1"]',
    }

    v21_extras = {
        "call_result_heartbeat": '[3,"1",{"currentTime":"2024-01-01T00:00:00.000Z"}]',
        "call_error_generic": '[4,"1","GenericError","x",{}]',
        "call_error_format_violation": '[4,"1","FormatViolation","x",{}]',
        "call_error_occurrence_constraint": '[4,"1","OccurrenceConstraintViolation","x",{}]',
        "call_error_rpc_framework": '[4,"1","RpcFrameworkError","x",{}]',
        "call_error_message_type_not_supported": '[4,"1","MessageTypeNotSupported","x",{}]',
        "call_error_not_implemented": '[4,"1","NotImplemented","x",{}]',
        "call_error_not_supported": '[4,"1","NotSupported","x",{}]',
        "call_error_internal": '[4,"1","InternalError","x",{}]',
        "call_error_protocol": '[4,"1","ProtocolError","x",{}]',
        "call_error_security": '[4,"1","SecurityError","x",{}]',
        "call_error_property_constraint": '[4,"1","PropertyConstraintViolation","x",{}]',
        "call_error_type_constraint": '[4,"1","TypeConstraintViolation","x",{}]',
        "call_error_unknown_code": '[4,"1","TotallyUnknownCode","x",{}]',
        "call_error_nested_details": '[4,"1","ProtocolError","x",{"nested":{"code":42}}]',
        "call_result_error_generic": '[5,"1","GenericError","x",{}]',
        "call_result_error_format_violation": '[5,"1","FormatViolation","x",{}]',
        "message_id_ok_36": '[2,"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","Heartbeat",{}]',
        "message_id_too_long": '[2,"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","Heartbeat",{}]',
        "call_null_payload": '[2,"1","Heartbeat",null]',
        "call_array_payload": '[2,"1","Heartbeat",[]]',
        "unknown_action": '[2,"1","NotARealAction",{}]',
        "wrong_arity": '[2,"1"]',
        "custom_data_extra": '[2,"1","Heartbeat",{"customData":{"vendorId":"V","extraKey":true}}]',
    }

    write_seeds("v16", v16, v16_extras)
    write_seeds("v21", v21, v21_extras)

    print(f"v16 CALL seeds: {len(v16)} + {len(v16_extras)} extras")
    print(f"v21 CALL/SEND seeds: {len(v21)} + {len(v21_extras)} extras")
    for ver, n_call, n_extra in (("v16", len(v16), len(v16_extras)), ("v21", len(v21), len(v21_extras))):
        for kind in ("deserialize", "roundtrip"):
            d = ROOT / "fuzz" / "corpus" / f"{ver}_{kind}"
            print(f"  {d.relative_to(ROOT)}: {len(list(d.glob('*.json')))} files")
        assert n_call + n_extra == len(list((ROOT / "fuzz" / "corpus" / f"{ver}_deserialize").glob("*.json")))


if __name__ == "__main__":
    main()
