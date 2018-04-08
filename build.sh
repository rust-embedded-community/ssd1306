#!/bin/sh

set -e

cargo build --target $TARGET --all-features --release

if [ -z $DISABLE_EXAMPLES ]; then
	cargo build --target $TARGET --all-features --examples
fi