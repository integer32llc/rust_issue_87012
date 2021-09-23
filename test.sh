#!/usr/bin/env bash
set -euo pipefail
shopt -s extglob

rm -f *.mm_profdata
cargo clean
RUSTFLAGS="-Zself-profile -Zself-profile-events=default,args" cargo +nightly build
crox repro_crate-*.mm_profdata
summarize summarize repro_crate-*.mm_profdata | grep evaluate
