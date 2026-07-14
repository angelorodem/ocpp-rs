#!/usr/bin/env bash
# Shared helpers for fuzz/run_*.sh (sourced, not executed directly).

run_fuzz_targets() {
  local seconds_per="$1"
  local jobs="$2"
  shift 2
  local targets=("$@")

  local root
  root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  cd "$root"

  if ! cargo +nightly fuzz --help >/dev/null 2>&1; then
    echo "cargo-fuzz not found. Install with: cargo +nightly install cargo-fuzz" >&2
    return 1
  fi

  local log_dir="$root/fuzz/logs"
  mkdir -p "$log_dir"

  echo "Repo:     $root"
  echo "Targets:  ${targets[*]}"
  echo "Duration: ${seconds_per}s each"
  echo "Jobs:     ${jobs} workers per target"
  echo "Logs:     $log_dir"
  echo

  local pids=()
  local t log
  for t in "${targets[@]}"; do
    log="$log_dir/${t}.log"
    echo "starting $t → $log"
    (
      cargo +nightly fuzz run "$t" --fuzz-dir fuzz -- \
        -max_total_time="$seconds_per" \
        -jobs="$jobs" \
        -workers="$jobs"
    ) >"$log" 2>&1 &
    pids+=("$!")
  done

  local fail=0
  local i pid ec
  for i in "${!targets[@]}"; do
    t="${targets[$i]}"
    pid="${pids[$i]}"
    if wait "$pid"; then
      echo "OK   $t (pid $pid)"
    else
      ec=$?
      echo "FAIL $t (pid $pid, exit $ec) — see $log_dir/${t}.log" >&2
      fail=1
    fi
  done

  if [[ "$fail" -ne 0 ]]; then
    echo >&2
    echo "One or more targets failed. Artifacts (if any): fuzz/artifacts/" >&2
    return 1
  fi

  echo
  echo "All fuzz targets finished successfully."
}
