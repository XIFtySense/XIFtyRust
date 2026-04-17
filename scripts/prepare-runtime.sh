#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNTIME_VERSION="${XIFTY_RUNTIME_VERSION:-$(cat "$ROOT/runtime-version.txt")}"
TARGET=""

if [ -n "${XIFTY_RUNTIME_TARGET:-}" ]; then
  TARGET="$XIFTY_RUNTIME_TARGET"
else
  case "$(uname -s)-$(uname -m)" in
    Darwin-arm64|Darwin-aarch64)
      TARGET="macos-arm64"
      ;;
    Linux-x86_64|Linux-amd64)
      TARGET="linux-x64"
      ;;
    *)
      echo "unsupported runtime host: $(uname -s) / $(uname -m)" >&2
      exit 1
      ;;
  esac
fi

CACHE_ROOT="${XIFTY_RUNTIME_CACHE_DIR:-$ROOT/.xifty-runtime}"
RUNTIME_DIR="$CACHE_ROOT/xifty-runtime-$TARGET-v$RUNTIME_VERSION"
ARCHIVE="$CACHE_ROOT/xifty-runtime-$TARGET-v$RUNTIME_VERSION.tar.gz"
RELEASE_TAG="${XIFTY_RUNTIME_RELEASE_TAG:-v$RUNTIME_VERSION}"
RUNTIME_URL="${XIFTY_RUNTIME_URL:-https://github.com/XIFtySense/XIFty/releases/download/$RELEASE_TAG/xifty-runtime-$TARGET-v$RUNTIME_VERSION.tar.gz}"

if [ -f "$RUNTIME_DIR/manifest.json" ]; then
  echo "Runtime already prepared at $RUNTIME_DIR"
  exit 0
fi

mkdir -p "$CACHE_ROOT"
rm -rf "$RUNTIME_DIR"
curl -LsSf "$RUNTIME_URL" -o "$ARCHIVE"
tar -xzf "$ARCHIVE" -C "$CACHE_ROOT"
test -f "$RUNTIME_DIR/manifest.json"
echo "Prepared XIFty runtime at $RUNTIME_DIR"
