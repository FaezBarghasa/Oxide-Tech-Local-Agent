#!/usr/bin/env bash
# Stage the oxide-embed sidecar for the Tauri bundler.
#
# Tauri `externalBin: ["binaries/oxide-embed"]` requires
#   src-tauri/binaries/oxide-embed-<rust-target-triple>
# at bundle time. This script resolves the binary in the same order as
# src-tauri/src/memory.rs and copies it into place (executable bit set).
#
# Resolution: $OXIDE_EMBED_BIN → ./bin/oxide-embed → PATH.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$ROOT/src-tauri/binaries"
TRIPLE="${TAURI_ENV_TARGET_TRIPLE:-$(rustc -vV 2>/dev/null | sed -n 's/^host: //p')}"
TRIPLE="${TRIPLE:-x86_64-unknown-linux-gnu}"
OUT="$OUT_DIR/oxide-embed-$TRIPLE"

resolve_bin() {
  if [[ -n "${OXIDE_EMBED_BIN:-}" && -f "$OXIDE_EMBED_BIN" ]]; then
    echo "$OXIDE_EMBED_BIN"; return 0
  fi
  if [[ -f "$ROOT/bin/oxide-embed" ]]; then
    echo "$ROOT/bin/oxide-embed"; return 0
  fi
  if command -v oxide-embed >/dev/null 2>&1; then
    command -v oxide-embed; return 0
  fi
  return 1
}

SRC="$(resolve_bin || true)"
if [[ -z "${SRC:-}" ]]; then
  echo "error: oxide-embed not found." >&2
  echo "  Set OXIDE_EMBED_BIN, drop the binary at ./bin/oxide-embed," >&2
  echo "  or install it on PATH (e.g. ~/.local/bin/oxide-embed)." >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
cp -f "$SRC" "$OUT"
chmod +x "$OUT"
echo "staged: $OUT  (from $SRC)"
"$OUT" --version 2>&1 | head -1 || true
