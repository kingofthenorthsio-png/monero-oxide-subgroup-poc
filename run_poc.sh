#!/usr/bin/env bash
set -euo pipefail
mkdir -p logs
ts=$(date +%Y%m%d-%H%M%S)
{
  echo "== Env =="; uname -a; git rev-parse HEAD; date
  rustc --version 2>/dev/null || true
  cargo --version 2>/dev/null || true
  forge --version 2>/dev/null || true
  node --version 2>/dev/null || true
  npm --version 2>/dev/null || true
} | tee "logs/env-$ts.log"

if [ -f Cargo.toml ]; then
  cargo test --all --verbose 2>&1 | tee "logs/cargo-test-$ts.log"
elif command -v forge >/dev/null 2>&1; then
  forge test -vvv 2>&1 | tee "logs/forge-test-$ts.log"
elif [ -f package.json ]; then
  npm test 2>&1 | tee "logs/npm-test-$ts.log"
else
  echo "No known test runner found." | tee "logs/unknown-$ts.log"
fi
