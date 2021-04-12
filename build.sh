#!/bin/sh

# Exit early on error
set -e

# Print commands as they're run
set -x

if [ -z $TARGET ]; then
    echo "TARGET environment variable required but not set"

    exit 1
fi

cargo fmt --all -- --check

cargo build --target $TARGET --all-features --release

cargo test --lib --target x86_64-unknown-linux-gnu
cargo test --doc --target x86_64-unknown-linux-gnu

if [ -z $DISABLE_EXAMPLES ]; then
    cargo build --target $TARGET --all-features --examples
fi

# Remove stale docs - the linkchecker might miss links to old files if they're not removed
cargo clean --doc
cargo clean --doc --target $TARGET

cargo doc --all-features --target $TARGET
