#!/bin/bash

DIR="$(dirname "$0")"
cargo build --release
# Copy to production target
[ -d "$DIR/target/release" ] && cp -r "$DIR/plugins" "$DIR/target/release/plugins"
[ -d "$DIR/target/release" ] && cp -r "$DIR/config.toml" "$DIR/target/release/"

# Copy to debug target
[ -d "$DIR/target/debug" ] && cp -r "$DIR/plugins" "$DIR/target/debug/plugins"

