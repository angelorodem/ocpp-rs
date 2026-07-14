# Datetime wire features

OCPP payloads store times as [`DateTimeWrapper`](../src/datetime.rs) (`chrono::DateTime<Utc>` in memory).

| Direction | Behavior |
|-----------|----------|
| **Parse** | Always RFC3339 via `DateTime::parse_from_rfc3339` (accepts `%.3fZ`, offsets like `+00:00`, variable fractional seconds, etc.) |
| **Serialize** | Default: strict `%Y-%m-%dT%H:%M:%S%.3fZ`. Optional feature below. |

## Feature

| Feature | Effect |
|---------|--------|
| *(none)* | Emit `%.3fZ` (recommended for older charge points) |
| `datetime_serialize_rfc3339` | Emit `to_rfc3339_opts(Millis, true)` |

```toml
# Default: accept RFC3339 inbound, emit %.3fZ outbound
ocpp-rs = { version = "0.4" }

# Emit RFC3339 millis on the wire as well:
ocpp-rs = { version = "0.4", features = ["datetime_serialize_rfc3339"] }
```

In-memory values are always `DateTime<Utc>`. You do not need a feature to “use RFC3339 in Rust code.”

Shared helpers: [`src/datetime.rs`](../src/datetime.rs). Re-exported as `v16::utils::iso8601_date_time` and `v21::utils::rfc3339_date_time` for existing `#[serde(with = …)]` paths (same implementation underneath).
