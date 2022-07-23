#!/usr/bin/env sh

set -eu
cargo build --release --target "$TARGET"
mkdir -p "./dist/"
cp "./target/$TARGET/$ARTIFACE" "./dist/$NAME"
