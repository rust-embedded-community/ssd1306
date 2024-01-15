#!/bin/sh

# Exit early on error
set -e

# Print commands as they're run
set -x

if [ -z $TARGET ]; then
    echo "TARGET environment variable required but not set"

    exit 1
fi

if [ -z $HOST_TARGET ]; then
    echo "HOST_TARGET environment variable is not set - Falling back to default"
fi
HOST_TARGET=${HOST_TARGET:-x86_64-unknown-linux-gnu}


cargo fmt --all -- --check

cargo build --target $TARGET --all-features --release

cargo test --lib --target $HOST_TARGET
cargo test --doc --target $HOST_TARGET

if [ -z $DISABLE_EXAMPLES ]; then
    cargo build --target $TARGET --all-features --examples
fi

# Remove stale docs - the linkchecker might miss links to old files if they're not removed
cargo clean --doc
cargo clean --doc --target $TARGET

cargo doc --all-features --target $TARGET
