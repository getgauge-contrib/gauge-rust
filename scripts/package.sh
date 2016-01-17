#!/bin/sh

# TODO: Replace this shell script with a cross-platfrom Rust script

set -e

cargo build --release

PROFILE="release"

if [ ! -z "$1" ]; then
    PROFILE="$1"
fi

PWD=`pwd`
BUILD_DIR="$PWD/build"
DEPLOY_RELATIVE_DIR="deploy"
DEPLOY_DIR="$PWD/$DEPLOY_RELATIVE_DIR"
TARGET_DIR="$PWD/target/$PROFILE"

ARCH=`uname -m`
PLATFORM=$(echo `uname` | tr '[:upper:]' '[:lower:]')
VERSION=`./target/release/gauge_rs --version`

OUTPUT_FILE="gauge-rs-$VERSION-$PLATFORM.$ARCH.zip"

rm -rf "$BUILD_DIR"
rm -rf "$DEPLOY_DIR"

mkdir -p "$BUILD_DIR/bin"
mkdir -p "$DEPLOY_DIR"

cp "$TARGET_DIR/gauge_rs" "$BUILD_DIR/bin/"
cp "rust.json" "$BUILD_DIR/"
cp "LICENSE" "$BUILD_DIR/"

pushd "$BUILD_DIR" > /dev/null
find . -path '*/.*' -prune -o -type f -print | zip -q -0 "$DEPLOY_DIR/$OUTPUT_FILE" -@
popd > /dev/null

echo "Create gauge-rs package in: $DEPLOY_RELATIVE_DIR/$OUTPUT_FILE"
echo "Install using:"
echo "\t$ gauge --install rust --file $DEPLOY_RELATIVE_DIR/$OUTPUT_FILE"
