# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --examples --target $TARGET
    cross build --examples --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # TODO: Uncomment when we have some tests
    # cross test --target $TARGET
    # cross test --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
