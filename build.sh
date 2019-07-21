#!/bin/sh

set -e

cargo build --target $TARGET --all-features --release

cargo test --lib --target x86_64-unknown-linux-gnu
cargo test --doc --target x86_64-unknown-linux-gnu

if [ -z $DISABLE_EXAMPLES ]; then
	cargo build --target $TARGET --all-features --examples
fi