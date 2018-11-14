set -exo pipefail

main() {
    if [ ! -z $COVERAGE ] && [ $TRAVIS_RUST_VERSION = nightly ]; then
        export CARGO_INCREMENTAL=0;
        export RUSTFLAGS="-Zprofile -Ccodegen-units=1";
    fi

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
        if [ ! -z $COVERAGE ]; then
            find . -name "*.gc*" -print
            zip -0 ccov.zip `find . \( -name "ads1x1x*.gc*" -o -name "tier*.gc*" -o -name "mux*.gc*" -o -name "construction*.gc*" -o -name "linux-*.gc*" -o -name "llvmgcov.gc*" \) -print`;
            grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" > lcov.info;
            bash <(curl -s https://codecov.io/bash) -f lcov.info;
            grcov ./target -t coveralls+ -s . --token `$COVERALLS_TOKEN` --commit-sha `$TRAVIS_COMMIT` --llvm --branch --ignore-not-existing --ignore-dir "/*"
        fi
    fi


}

main
