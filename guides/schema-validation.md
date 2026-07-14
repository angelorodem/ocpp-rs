# Schema validation

Enable with:

```toml
ocpp-rs = { version = "0.4", features = ["schema_validate"] }
```

When enabled, [`deserialize_to_message`](../src/v21/parse.rs) (v16 and v21) runs generated
Part 3 / 1.6 constraint checks on CALL (and v21 SEND) payloads after serde succeeds.

## What is checked

- `maxLength` on strings
- `minItems` / `maxItems` on arrays
- `minimum` / `maximum` on numbers

Shape validation (`required`, enums, `additionalProperties: false`) remains serde +
`deny_unknown_fields`.

## Errors → RPC codes

Map [`Error::ConstraintViolation`](../src/errors.rs) to CALLERROR:

| Kind | Suggested RPC code (2.1) | Suggested RPC code (1.6) |
|------|--------------------------|---------------------------|
| `MaxLength` / `Minimum` / `Maximum` | `PropertyConstraintViolation` | same |
| `MinItems` / `MaxItems` | `OccurrenceConstraintViolation` | `OccurenceConstraintViolation` |
| `MessageIdTooLong` | `RpcFrameworkError` or `FormatViolation` | `FormationViolation` |

Constructors: `CallError::property_constraint_violation`, `occurrence_constraint_violation`
(v21) / `occurence_constraint_violation` (v16).

## Regenerating validators

```bash
python tools/gen_validate.py
python tools/gen_validate.py --check   # CI
```

Sources: `docs/2-1-raw/schemas/`, `docs/1-6-raw/schemas/` (local extracts).
