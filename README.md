# Rust Client for Grafana Loki 

## Endpoint Coverage

| Method | Path | Implemented? |
| --- | --- | --- |
| `GET` | `/ready` | ✅ |
| `GET` | `/metrics` | N/A |
| `GET` | `/config` | |
| `GET` | `/services` | ✅ |
| `GET` | `/flush` | ✅ |
| `GET` | `/ingester/shutdown` | ✅ |
| `GET` | `/loki/api/v1/status/buildinfo` | ✅ |
| `GET` | `/loki/api/v1/format_query` | |
| `GET` | `/loki/api/v1/query` | |
| `GET` | `/loki/api/v1/query_range` | |
| `GET` | `/loki/api/v1/labels` | |
| `GET` | `/loki/api/v1/label/<name>/values` | |
| `GET` | `/loki/api/v1/series` | |
| `GET` | `/loki/api/v1/index/stats` | |
| `GET` | `/loki/api/v1/tail` | |
| `POST` | `/loki/api/v1/push` | ✅ |
