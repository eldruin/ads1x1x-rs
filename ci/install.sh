set -exo pipefail

main() {
    rustup component add rust-src
    SYSROOT=$(rustc --print sysroot)
    if [[ ! "$SYSROOT" =~ "$TARGET" ]]; then
        rustup target add $TARGET
    else
        echo "Target $TARGET is already installed"
    fi
    if [ ! -z $COVERAGE ]; then
        if ! [ -x "$(command -v grcov)" ]; then
            cargo install grcov
        else
            echo "grcov is already installed"
        fi
    fi
}

main
