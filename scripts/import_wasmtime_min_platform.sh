#!/usr/bin/env bash
set -euo pipefail

# Import the Wasmtime min-platform example into this repository.
# This script is intentionally explicit so it can be re-run when upstream changes.

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
DEST_DIR="$ROOT_DIR/third_party/wasmtime-min-platform"
TMP_DIR=$(mktemp -d)
REPO_URL="https://github.com/bytecodealliance/wasmtime"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "Cloning $REPO_URL ..."
git clone --depth 1 "$REPO_URL" "$TMP_DIR/wasmtime"

SRC_DIR="$TMP_DIR/wasmtime/examples/min-platform"
if [[ ! -d "$SRC_DIR" ]]; then
  echo "error: source directory not found: $SRC_DIR" >&2
  exit 1
fi

rm -rf "$DEST_DIR"
mkdir -p "$DEST_DIR"
cp -a "$SRC_DIR"/. "$DEST_DIR"/

echo "Imported into: $DEST_DIR"
