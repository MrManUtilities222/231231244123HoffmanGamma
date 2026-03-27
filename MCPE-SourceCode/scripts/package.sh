#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
RP_DIR="$ROOT_DIR/rust-port"
DATA_DIR="$ROOT_DIR/data"
DIST_DIR="$ROOT_DIR/dist"

if [ ! -d "$RP_DIR" ]; then
  echo "Expecting to be inside repository. Couldn't find $RP_DIR" >&2
  exit 1
fi

TARGET=${1:-}
if [ -z "$TARGET" ]; then
  UNAME=$(uname -s)
  case "$UNAME" in
    Linux*) TARGET=linux ;;
    Darwin*) TARGET=macos ;;
    MINGW*|MSYS*|CYGWIN*) TARGET=windows ;;
    *) TARGET=linux ;;
  esac
fi

cd "$RP_DIR"
cargo build --release

mkdir -p "$DIST_DIR"

case "$TARGET" in
  linux)
    PKG="$DIST_DIR/rustcraft-linux-x86_64"
    rm -rf "$PKG"
    mkdir -p "$PKG"
    cp target/release/rustcraft "$PKG/"
    cp -r "$DATA_DIR" "$PKG/data"
    tar -C "$DIST_DIR" -czf "$DIST_DIR/rustcraft-linux-x86_64.tar.gz" "$(basename "$PKG")"
    echo "Packaged: $DIST_DIR/rustcraft-linux-x86_64.tar.gz"
    ;;
  macos)
    PKG="$DIST_DIR/rustcraft-macos"
    rm -rf "$PKG"
    mkdir -p "$PKG"
    cp target/release/rustcraft "$PKG/"
    cp -r "$DATA_DIR" "$PKG/data"
    if command -v hdiutil >/dev/null 2>&1; then
      DMG="$DIST_DIR/rustcraft-macos.dmg"
      hdiutil create -volname "RustCraft" -srcfolder "$PKG" -ov -format UDZO "$DMG"
      echo "Packaged DMG: $DMG"
    else
      ZIP="$DIST_DIR/rustcraft-macos.zip"
      (cd "$DIST_DIR" && zip -r "$(basename "$ZIP")" "$(basename "$PKG")")
      echo "Packaged ZIP: $ZIP (hdiutil missing)"
    fi
    ;;
  windows)
    PKG="$DIST_DIR/rustcraft-windows-x86_64"
    rm -rf "$PKG"
    mkdir -p "$PKG"
    cp target/release/rustcraft.exe "$PKG/" || cp target/release/rustcraft "$PKG/"
    cp -r "$DATA_DIR" "$PKG/data"
    ZIP="$DIST_DIR/rustcraft-windows-x86_64.zip"
    (cd "$DIST_DIR" && zip -r "$(basename "$ZIP")" "$(basename "$PKG")")
    echo "Packaged ZIP: $ZIP"
    ;;
  *)
    echo "Unknown target: $TARGET" >&2
    exit 1
    ;;
esac
