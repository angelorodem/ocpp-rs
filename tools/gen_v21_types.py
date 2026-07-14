#!/usr/bin/env python3
"""One-shot bootstrap: OCPP 2.1 JSON schemas -> checked-in Rust sources under src/v21/.

Not a build.rs codegen step — output is committed and maintained as hand-editable source.
"""
from __future__ import annotations

import json
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "docs" / "2-1-raw" / "schemas"
OUT = ROOT / "src" / "v21"

SKIP_SHARED = {"CustomDataType"}  # always import from datatypes


def snake(name: str) -> str:
    s1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


def rust_ident(name: str) -> str:
    reserved = {
        "type",
        "match",
        "ref",
        "self",
        "super",
        "crate",
        "async",
        "await",
        "try",
        "use",
        "mod",
        "fn",
        "let",
        "pub",
        "enum",
        "struct",
        "impl",
        "where",
        "as",
        "in",
        "if",
        "else",
        "loop",
        "while",
        "for",
        "return",
        "break",
        "continue",
        "const",
        "static",
        "trait",
        "move",
        "mut",
        "dyn",
        "box",
        "true",
        "false",
    }
    s = snake(name)
    if s in reserved:
        return f"{s}_"
    if s[0].isdigit():
        return f"n_{s}"
    return s


def enum_variant(val: str) -> str:
    """Rust enum variant in UpperCamelCase; original wire value kept via serde rename."""
    parts = re.split(r"[^A-Za-z0-9]+", val)
    parts = [p for p in parts if p]
    if not parts:
        return "Unknown"
    camel = "".join(p[:1].upper() + p[1:] for p in parts)
    if camel[0].isdigit():
        camel = f"V{camel}"
    return camel


def collect_schemas():
    requests = {}
    responses = {}
    send = None
    for f in sorted(SCHEMA_DIR.glob("*.json")):
        name = f.stem
        data = json.loads(f.read_text())
        if name.endswith("Request"):
            requests[name[: -len("Request")]] = data
        elif name.endswith("Response"):
            responses[name[: -len("Response")]] = data
        elif name == "NotifyPeriodicEventStream":
            send = data
    return requests, responses, send


def enum_values(defn: dict) -> tuple[str, ...]:
    return tuple(defn.get("enum") or ())


def struct_sig(defn: dict) -> str:
    """Structure-only signature for dedup (ignore descriptions)."""
    if defn.get("enum"):
        return "enum:" + ",".join(defn["enum"])
    if defn.get("type") != "object":
        return json.dumps(defn, sort_keys=True)
    props = defn.get("properties") or {}
    parts = []
    for k in sorted(props):
        p = props[k]
        if "$ref" in p:
            parts.append(f"{k}->${p['$ref'].split('/')[-1]}")
        else:
            t = p.get("type", "?")
            fmt = p.get("format", "")
            parts.append(f"{k}:{t}:{fmt}")
    req = ",".join(sorted(defn.get("required") or []))
    return f"obj[{';'.join(parts)}]|req={req}"


def collect_shared_defs(requests, responses, send):
    """Return name -> canonical def for defs that appear identically (by sig) in 2+ files."""
    by_name: dict[str, dict[str, list]] = defaultdict(lambda: defaultdict(list))
    all_schemas = list(requests.values()) + list(responses.values())
    if send:
        all_schemas.append(send)
    for schema in all_schemas:
        for n, d in (schema.get("definitions") or {}).items():
            if n in SKIP_SHARED:
                continue
            by_name[n][struct_sig(d)].append(d)

    shared = {}
    for n, variants in by_name.items():
        # Prefer the variant that appears most often; if single signature, share it
        best_sig, defs = max(variants.items(), key=lambda kv: len(kv[1]))
        total = sum(len(v) for v in variants.values())
        if len(variants) == 1 and total >= 2:
            shared[n] = defs[0]
        elif n.endswith("EnumType") and len(variants) > 1:
            # Union enum values across variants
            vals = []
            seen = set()
            for defs_list in variants.values():
                for d in defs_list:
                    for v in d.get("enum") or []:
                        if v not in seen:
                            seen.add(v)
                            vals.append(v)
            shared[n] = {
                "type": "string",
                "enum": vals,
                "additionalProperties": False,
            }
        elif total >= 3 and len(variants) == 1:
            shared[n] = defs[0]
    # Always share these common types if present
    for force in (
        "StatusInfoType",
        "EVSEType",
        "ComponentType",
        "VariableType",
        "IdTokenType",
        "AdditionalInfoType",
        "CertificateHashDataType",
        "MessageContentType",
    ):
        if force in by_name and force not in shared:
            # take most common
            best = max(by_name[force].items(), key=lambda kv: len(kv[1]))
            shared[force] = best[1][0]
    return shared


def ref_name(ref: str) -> str:
    return ref.split("/")[-1]


def rust_type_for_prop(
    prop: dict,
    shared: dict,
    local_names: set[str],
    msg_prefix: str,
) -> tuple[str, bool]:
    """Return (rust_type, needs_datetime_attr)."""
    if "$ref" in prop:
        name = ref_name(prop["$ref"])
        if name == "CustomDataType":
            return "CustomDataType", False
        if name in shared:
            return name, False
        # local type — may be renamed with prefix if conflict handled elsewhere
        return name, False

    t = prop.get("type")
    if isinstance(t, list):
        # nullable etc — take first non-null
        t = next((x for x in t if x != "null"), t[0])

    if t == "array":
        items = prop.get("items") or {}
        inner, _ = rust_type_for_prop(items, shared, local_names, msg_prefix)
        return f"Vec<{inner}>", False
    if t == "string":
        if prop.get("format") == "date-time":
            return "DateTimeWrapper", True
        return "String", False
    if t == "integer":
        return "i32", False
    if t == "number":
        return "f64", False
    if t == "boolean":
        return "bool", False
    if t == "object":
        # free-form object (e.g. data, errorDetails-like)
        return "serde_json::Value", False
    # fallback
    return "serde_json::Value", False


def gen_enum(name: str, defn: dict) -> str:
    variants = []
    for v in defn.get("enum") or []:
        rv = enum_variant(v)
        variants.append(f'    #[serde(rename = "{v}")]\n    {rv},')
    body = "\n".join(variants)
    return f"""#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum {name} {{
{body}
}}
"""


def gen_struct(
    name: str,
    defn: dict,
    shared: dict,
    local_names: set[str],
    msg_prefix: str,
    deny_unknown: bool = True,
) -> str:
    props = defn.get("properties") or {}
    required = set(defn.get("required") or [])
    # CustomDataType is special — allow unknown for extensions? plan says vendorId only
    if name == "CustomDataType":
        deny_unknown = False

    fields = []
    uses_dt = False
    for pname, prop in props.items():
        rust_name = rust_ident(pname)
        rtype, is_dt = rust_type_for_prop(prop, shared, local_names, msg_prefix)
        if is_dt:
            uses_dt = True
        is_req = pname in required
        attrs = []
        if rust_name != snake(pname) or rust_name.endswith("_"):
            # always rename if we changed the ident from camelCase conversion of pname
            if rust_ident(pname) != snake(pname) or pname == "type":
                attrs.append(f'#[serde(rename = "{pname}")]')
        elif snake(pname) != rust_name:
            attrs.append(f'#[serde(rename = "{pname}")]')

        # serde rename_all camelCase handles field names; only need rename for reserved
        if pname == "type" or rust_name.endswith("_") and snake(pname) + "_" == rust_name:
            attrs = [f'#[serde(rename = "{pname}")]']

        if not is_req:
            attrs.append('#[serde(skip_serializing_if = "Option::is_none")]')
            attrs.append("#[serde(default)]")
            rtype = f"Option<{rtype}>"
        if is_dt:
            if is_req:
                attrs.append('#[serde(with = "crate::v21::utils::rfc3339_date_time")]')
            else:
                attrs.append('#[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]')
                # already Option
        attr_s = ("\n    ".join(attrs) + "\n    ") if attrs else ""
        fields.append(f"    {attr_s}pub {rust_name}: {rtype},")

    deny = ', deny_unknown_fields' if deny_unknown and name != "CustomDataType" else ""
    # PartialEq: f64/Value break Eq — use PartialEq only always
    derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
    body = "\n".join(fields) if fields else ""
    return f"""#[derive({derives})]
#[serde(rename_all = "camelCase"{deny})]
pub struct {name} {{
{body}
}}
"""


def gather_local_defs(schema: dict, shared: dict) -> dict:
    result = {}
    for n, d in (schema.get("definitions") or {}).items():
        if n in SKIP_SHARED:
            continue
        if n in shared and struct_sig(d) == struct_sig(shared[n]):
            continue
        if n in shared and n.endswith("EnumType"):
            # using unified shared enum
            continue
        if n in shared:
            # conflicting object — keep local with same name only if identical; else rename
            if struct_sig(d) != struct_sig(shared[n]):
                result[n] = d  # will emit locally; may shadow — use local in this file only
            continue
        result[n] = d
    return result


def topo_sort_defs(defs: dict) -> list[str]:
    """Order defs so refs to local defs come first."""
    names = set(defs)
    deps = {n: set() for n in names}

    def walk(obj, acc):
        if isinstance(obj, dict):
            if "$ref" in obj:
                r = ref_name(obj["$ref"])
                if r in names:
                    acc.add(r)
            for v in obj.values():
                walk(v, acc)
        elif isinstance(obj, list):
            for v in obj:
                walk(v, acc)

    for n, d in defs.items():
        acc = set()
        walk(d, acc)
        deps[n] = acc - {n}

    ordered = []
    remaining = set(names)
    while remaining:
        ready = [n for n in remaining if deps[n] <= set(ordered)]
        if not ready:
            # cycle — just flush
            ready = sorted(remaining)
        for n in sorted(ready):
            ordered.append(n)
            remaining.discard(n)
            break
        else:
            break
    return ordered


def gen_message_file(action: str, req: dict, resp: dict, shared: dict) -> str:
    local = {}
    local.update(gather_local_defs(req, shared))
    local.update(gather_local_defs(resp, shared))
    # If same name in both with different sig — last wins; rare after enum merge

    imports = {
        "use serde::{Deserialize, Serialize};",
        "use crate::v21::datatypes::CustomDataType;",
    }
    # scan for shared refs
    def scan(obj):
        if isinstance(obj, dict):
            if "$ref" in obj:
                yield ref_name(obj["$ref"])
            for v in obj.values():
                yield from scan(v)
        elif isinstance(obj, list):
            for v in obj:
                yield from scan(v)

    used = set(scan(req)) | set(scan(resp))
    for n in used:
        if n in shared:
            if n.endswith("EnumType"):
                imports.add(f"use crate::v21::enumerations::{n};")
            else:
                imports.add(f"use crate::v21::datatypes::{n};")
        if n == "CustomDataType":
            imports.add("use crate::v21::datatypes::CustomDataType;")

    needs_dt = False

    def check_dt(obj):
        nonlocal needs_dt
        if isinstance(obj, dict):
            if obj.get("format") == "date-time":
                needs_dt = True
            for v in obj.values():
                check_dt(v)
        elif isinstance(obj, list):
            for v in obj:
                check_dt(v)

    check_dt(req)
    check_dt(resp)
    check_dt(local)
    if needs_dt:
        imports.add("use crate::v21::datatypes::DateTimeWrapper;")

    parts = ["//! OCPP 2.1 " + action + " request/response payloads.\n"]
    parts.append("\n".join(sorted(imports)))
    parts.append("")

    local_names = set(local)
    for n in topo_sort_defs(local):
        d = local[n]
        if d.get("enum"):
            parts.append(gen_enum(n, d))
        else:
            parts.append(gen_struct(n, d, shared, local_names, action))

    parts.append(gen_struct(f"{action}Request", req, shared, local_names, action))
    parts.append(gen_struct(f"{action}Response", resp, shared, local_names, action))
    body = "\n".join(parts)
    extra = []
    if re.search(r"\bString\b", body):
        extra.append("use alloc::string::String;")
    if re.search(r"\bVec<", body):
        extra.append("use alloc::vec::Vec;")
    if extra:
        # insert after doc comment line
        lines = body.split("\n")
        # find first use line
        idx = next((i for i,l in enumerate(lines) if l.startswith("use ")), 1)
        for e in reversed(extra):
            if e not in body:
                lines.insert(idx, e)
        body = "\n".join(lines)
    return body


def gen_send_file(schema: dict, shared: dict) -> str:
    action = "NotifyPeriodicEventStream"
    local = gather_local_defs(schema, shared)
    imports = [
        "use serde::{Deserialize, Serialize};",
        "use crate::v21::datatypes::CustomDataType;",
        "use crate::v21::datatypes::DateTimeWrapper;",
        "use alloc::string::String;",
        "use alloc::vec::Vec;",
    ]
    for n in set():
        pass
    used = set()
    def scan(obj):
        if isinstance(obj, dict):
            if "$ref" in obj:
                used.add(ref_name(obj["$ref"]))
            for v in obj.values():
                scan(v)
        elif isinstance(obj, list):
            for v in obj:
                scan(v)
    scan(schema)
    for n in used:
        if n in shared:
            if n.endswith("EnumType"):
                imports.append(f"use crate::v21::enumerations::{n};")
            else:
                imports.append(f"use crate::v21::datatypes::{n};")

    parts = [f"//! OCPP 2.1 SEND payload: {action}.\n", "\n".join(sorted(set(imports))), ""]
    for n in topo_sort_defs(local):
        d = local[n]
        if d.get("enum"):
            parts.append(gen_enum(n, d))
        else:
            parts.append(gen_struct(n, d, shared, set(local), action))
    parts.append(gen_struct(action, schema, shared, set(local), action))
    return "\n".join(parts)


def write_shared(shared: dict):
    enum_dir = OUT / "enumerations"
    data_dir = OUT / "datatypes"
    enum_dir.mkdir(parents=True, exist_ok=True)
    data_dir.mkdir(parents=True, exist_ok=True)

    enums = []
    datas = ["custom_data", "date_time"]

    # CustomDataType
    (data_dir / "custom_data.rs").write_text(
        """//! Vendor extension envelope (`customData`).
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataType {
    pub vendor_id: String,
}
"""
    )

    (data_dir / "date_time.rs").write_text(
        """//! RFC3339 datetime wrapper for OCPP 2.1 payloads.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeWrapper(DateTime<Utc>);

impl DateTimeWrapper {
    #[must_use]
    pub const fn new(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    #[must_use]
    pub const fn inner(&self) -> DateTime<Utc> {
        self.0
    }
}
"""
    )

    for n, d in sorted(shared.items()):
        if d.get("enum"):
            path = enum_dir / f"{snake(n)}.rs"
            path.write_text(f"//! {n}\nuse serde::{{Deserialize, Serialize}};\n\n" + gen_enum(n, d))
            enums.append(snake(n))
        else:
            path = data_dir / f"{snake(n)}.rs"
            # imports
            imports = [
                "use serde::{Deserialize, Serialize};",
                "use super::CustomDataType;",
            ]
            body_needs = set()
            def scan(obj):
                if isinstance(obj, dict):
                    if "$ref" in obj:
                        body_needs.add(ref_name(obj["$ref"]))
                    if obj.get("format") == "date-time":
                        body_needs.add("DateTimeWrapper")
                    for v in obj.values():
                        scan(v)
                elif isinstance(obj, list):
                    for v in obj:
                        scan(v)
            scan(d)
            for bn in body_needs:
                if bn == "CustomDataType":
                    continue
                if bn == "DateTimeWrapper":
                    imports.append("use super::DateTimeWrapper;")
                elif bn.endswith("EnumType"):
                    imports.append(f"use crate::v21::enumerations::{bn};")
                elif bn in shared:
                    imports.append(f"use super::{bn};")
            content = (
                f"//! {n}\n"
                + "\n".join(sorted(set(imports)))
                + "\n\n"
                + gen_struct(n, d, shared, set(), n)
            )
            path.write_text(content)
            datas.append(snake(n))

    # mod files
    enum_mod = ["//! OCPP 2.1 enumerations.\n"]
    for e in sorted(set(enums)):
        enum_mod.append(f"pub mod {e};")
        # re-export
    enum_mod.append("")
    for e in sorted(set(enums)):
        # find type name from file — PascalCase from snake is hard; read first enum name
        text = (enum_dir / f"{e}.rs").read_text()
        m = re.search(r"pub enum (\w+)", text)
        if m:
            enum_mod.append(f"pub use {e}::{m.group(1)};")
    (enum_dir / "mod.rs").write_text("\n".join(enum_mod) + "\n")

    data_mod = ["//! OCPP 2.1 shared datatypes.\n", "pub mod custom_data;", "pub mod date_time;"]
    for e in sorted(set(datas)):
        if e in ("custom_data", "date_time"):
            continue
        data_mod.append(f"pub mod {e};")
    data_mod.append("")
    data_mod.append("pub use custom_data::CustomDataType;")
    data_mod.append("pub use date_time::DateTimeWrapper;")
    for e in sorted(set(datas)):
        if e in ("custom_data", "date_time"):
            continue
        text = (data_dir / f"{e}.rs").read_text()
        m = re.search(r"pub struct (\w+)", text)
        if m:
            data_mod.append(f"pub use {e}::{m.group(1)};")
    (data_dir / "mod.rs").write_text("\n".join(data_mod) + "\n")
    return enums, datas


def gen_action_dispatch(actions: list[str]) -> str:
    """Generate call.rs Action enum + deserialize visitor skeleton pieces."""
    variants = "\n".join(f"    {a}(crate::v21::messages::{snake(a)}::{a}Request)," for a in actions)
    match_arms = []
    for a in actions:
        match_arms.append(
            f'            "{a}" => {{\n'
            f"                let payload = map.next_value()?;\n"
            f"                Action::{a}(payload)\n"
            f"            }}"
        )
    return variants, "\n".join(match_arms)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    msg_dir = OUT / "messages"
    msg_dir.mkdir(parents=True, exist_ok=True)

    requests, responses, send = collect_schemas()
    actions = sorted(set(requests) & set(responses))
    missing_resp = set(requests) - set(responses)
    missing_req = set(responses) - set(requests)
    if missing_resp or missing_req:
        print("WARNING missing pairs", missing_resp, missing_req)

    shared = collect_shared_defs(requests, responses, send)
    print(f"Shared defs: {len(shared)}")
    write_shared(shared)

    msg_mods = []
    for action in actions:
        content = gen_message_file(action, requests[action], responses[action], shared)
        path = msg_dir / f"{snake(action)}.rs"
        path.write_text(content)
        msg_mods.append(snake(action))
        print("wrote", path.name)

    if send:
        path = msg_dir / "notify_periodic_event_stream.rs"
        path.write_text(gen_send_file(send, shared))
        msg_mods.append("notify_periodic_event_stream")
        print("wrote notify_periodic_event_stream.rs")

    mod = ["//! OCPP 2.1 message payloads.\n"]
    for m in sorted(set(msg_mods)):
        mod.append(f"pub mod {m};")
    (msg_dir / "mod.rs").write_text("\n".join(mod) + "\n")

    # write actions list for the wire layer
    (OUT / "actions.txt").write_text("\n".join(actions) + "\n")
    print(f"Done: {len(actions)} actions")


if __name__ == "__main__":
    main()
