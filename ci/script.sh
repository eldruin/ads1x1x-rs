set -exo pipefail

main() {
    if [[ ! $TARGET =~ .*linux.* ]]; then
        sed -i "s/linux-embedded-hal/#linux-embedded-hal/g" Cargo.toml
        sed -i "s/embedded-hal-mock/#embedded-hal-mock/g" Cargo.toml
    fi

    if [ ! -z $FEATURES ]; then
       export FEATURES="--features $FEATURES"
    fi

    cargo check --target $TARGET $FEATURES
    cargo build --target $TARGET --release $FEATURES
    if [ -z $DISABLE_EXAMPLES ] && [[ $TARGET =~ .*linux.* ]]; then
        cargo build --target $TARGET $FEATURES --examples
    fi

    if [ -z $DISABLE_TESTS ] && [ $TRAVIS_RUST_VERSION = nightly ] && [[ $TARGET =~ .*linux.* ]]; then
        cargo test --target $TARGET $FEATURES
    fi
}

main
