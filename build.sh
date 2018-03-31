#!/bin/sh

set  -e

xargo build --target $TARGET --all-features
xargo build --target $TARGET --all-features --release

if [ -z $DISABLE_EXAMPLES ]; then
	xargo build --target $TARGET --all-features --examples
	xargo build --target $TARGET --all-features --examples
fi
