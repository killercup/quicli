#! /bin/bash

set -e
CARGO_TARGET_DIR=(pwd)

for file in docs/*.md; {
    TESTNAME="$(basename $file .md)"
    if [ -z "$1" ] || [ "$1" == "$TESTNAME" ]; then
        echo "Checking $TESTNAME"
        TARGET="examples/$TESTNAME"
        rm "$TARGET/Cargo.toml" || true
        rm "$TARGET/src/main.rs" || true
        waltz "$file" -o $TARGET
    fi
}

if [[ `git status --porcelain` ]]; then
    echo "Code in guides and corresponding examples differs!"
    if [ -z "$CI" ]; then
        echo "The files have been updated. Please check in these changes."
    else
        echo "Please run \`doc/_test.sh\` and check in the changes:"
        git diff
        exit 1
    fi
else
    echo "all examples are up to date with the guides"
fi
