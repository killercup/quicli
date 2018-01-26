#! /bin/bash

set -e
rm -r generated_doctests/ || true

for file in docs/*.md; {(\
    TARGET="generated_doctests/$(basename $file .md)"

    waltz "$file" -o $TARGET

    cd $TARGET
    cargo check
)}
