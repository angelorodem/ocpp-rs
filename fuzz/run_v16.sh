#!/usr/bin/env bash
# Run all OCPP 1.6 fuzz targets in parallel.
#
# Usage:
#   ./fuzz/run_v16.sh
#   ./fuzz/run_v16.sh 600       # seconds per target (default: 300)
#   ./fuzz/run_v16.sh 300 8     # seconds, jobs/workers per target (default: 4)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=run_common.sh
source "$ROOT/fuzz/run_common.sh"

run_fuzz_targets \
  "${1:-300}" \
  "${2:-4}" \
  v16_deserialize \
  v16_roundtrip \
  v16_structured \
  v16_corrupt
