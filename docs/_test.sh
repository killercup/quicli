#! /bin/bash

set -e
CARGO_TARGET_DIR=(pwd)

mkdir -pv generated_doctests/ || true

run_test () {
    TARGET="generated_doctests/$TESTNAME"
    rm -rf "$TARGET" || true;

    waltz "$file" -o $TARGET

    cargo check --quiet --manifest-path "$TARGET/Cargo.toml"
}

for file in docs/*.md; {
    TESTNAME="$(basename $file .md)"
    if [ -z "$1" ] || [ "$1" == "$TESTNAME" ]; then
        echo "Checking $TESTNAME"
        run_test
    fi
}
