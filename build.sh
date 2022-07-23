#!/usr/bin/env sh

set -eu
cargo build --release --target "$TARGET"
mkdir -p "./dist/"
cp "./target/$TARGET/release/$ARTIFACT" "./dist/$NAME"
