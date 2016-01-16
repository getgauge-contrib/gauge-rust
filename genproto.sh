#!/bin/sh

set -e

if [ -z "$PROTOC_GEN_RUST_PATH" ]; then
    echo "PROTOC_GEN_RUST_PATH environment variable needs to be set to \"<path-to-rust-protobuf>/target/debug\""
    echo "See: https://github.com/stepancheg/rust-protobuf"
    exit 1
fi

if [ ! -x "$PROTOC_GEN_RUST_PATH/protoc-gen-rust" ]; then
    echo "protoc-gen-rust not found in PROTOC_GEN_RUST_PATH or is not executable"
    exit 1
fi

command -v protoc >/dev/null 2>&1 || { echo >&2 "protoc executable is not installed or is not on PATH."; exit 1; }

PATH="$PROTOC_GEN_RUST_PATH:$PATH"

DEST_DIR="../proto"
GAUGE_PROTO_DIR="gauge-proto"

protoc --version

#Using protoc version 2.5.0
cd "$GAUGE_PROTO_DIR"

rm -rf "$DEST_DIR"
mkdir -p "$DEST_DIR"

protoc --rust_out ../proto/ "spec.proto"
protoc --rust_out ../proto/ "messages.proto"
protoc --rust_out ../proto/ "api.proto"
