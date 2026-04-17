#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="${XIFTY_CORE_DIR:-$ROOT/.xifty-core}"
CORE_REPO="${XIFTY_CORE_REPO:-https://github.com/XIFtySense/XIFty.git}"
CORE_REF="${XIFTY_CORE_REF:-main}"

if [ ! -d "$CORE_DIR/.git" ]; then
  git clone --depth 1 --branch "$CORE_REF" "$CORE_REPO" "$CORE_DIR"
else
  git -C "$CORE_DIR" fetch --depth 1 origin "$CORE_REF"
  git -C "$CORE_DIR" checkout --force FETCH_HEAD
fi

cargo build -p xifty-ffi --manifest-path "$CORE_DIR/Cargo.toml"
echo "Prepared XIFty core at $CORE_DIR"
