#!/usr/bin/env bash
# Remove cargo-fuzz runtime outputs (keeps seed corpus *.json and source).
#
# Usage:
#   ./fuzz/clean.sh                # everything: artifacts, logs, hash corpus, fuzz/target
#   ./fuzz/clean.sh --keep-corpus  # leave libFuzzer hash inputs in fuzz/corpus/
#   ./fuzz/clean.sh --keep-target  # leave fuzz/target build cache

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FUZZ="$ROOT/fuzz"

KEEP_CORPUS=0
KEEP_TARGET=0
for arg in "$@"; do
  case "$arg" in
    --keep-corpus) KEEP_CORPUS=1 ;;
    --keep-target) KEEP_TARGET=1 ;;
    --corpus|--target|--all)
      # backward-compatible no-ops: full clean is the default
      ;;
    -h|--help)
      cat <<'EOF'
Remove cargo-fuzz runtime outputs (keeps seed corpus *.json and source).

Usage:
  ./fuzz/clean.sh                # artifacts, coverage, logs, hash corpus, fuzz/target
  ./fuzz/clean.sh --keep-corpus  # do not delete discovered hash inputs
  ./fuzz/clean.sh --keep-target  # do not delete fuzz/target build cache
EOF
      exit 0
      ;;
    *)
      echo "unknown option: $arg (try --help)" >&2
      exit 1
      ;;
  esac
done

rm -rf "$FUZZ/artifacts" "$FUZZ/coverage"
mkdir -p "$FUZZ/artifacts"

# Per-target / job logs
rm -rf "$FUZZ/logs"
rm -f "$ROOT"/fuzz-*.log "$FUZZ"/fuzz-*.log

# libFuzzer sometimes drops crash-* / hang-* beside the binary or in cwd
find "$FUZZ" -maxdepth 3 \( -name 'crash-*' -o -name 'hang-*' -o -name '*.log' \) \
  ! -path "$FUZZ/fuzz_targets/*" \
  -type f -delete 2>/dev/null || true

if [[ "$KEEP_CORPUS" -eq 0 ]]; then
  echo "Removing discovered corpus inputs (keeping *.json seeds)…"
  if [[ -d "$FUZZ/corpus" ]]; then
    find "$FUZZ/corpus" -type f ! -name '*.json' -delete
  fi
fi

if [[ "$KEEP_TARGET" -eq 0 ]]; then
  echo "Removing fuzz/target build cache…"
  rm -rf "$FUZZ/target"
fi

echo "Cleaned fuzz runtime outputs under $FUZZ"
echo "  artifacts/ coverage/ logs/  (and repo-root fuzz-*.log)"
[[ "$KEEP_CORPUS" -eq 0 ]] && echo "  + non-json corpus entries (hash inputs)"
[[ "$KEEP_CORPUS" -eq 1 ]] && echo "  (kept discovered corpus)"
[[ "$KEEP_TARGET" -eq 0 ]] && echo "  + fuzz/target"
[[ "$KEEP_TARGET" -eq 1 ]] && echo "  (kept fuzz/target)"
