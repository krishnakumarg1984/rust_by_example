#!/usr/bin/env bash

printf "Deleting compiled executables and other build artefacts ...\n"

find . -type f -name "Cargo.lock" -print -delete 2>/dev/null
find . -type f \( ! -name "*.*" \) \( -perm -u=x \) -print -delete 2>/dev/null
find . -type d \( -name "target" -o -name "debug" -o -name "release" \) -print -exec rm -rf "{}" \; 2>/dev/null
find . -type d -name "build" -print -exec rm -rf "{}" \; 2>/dev/null

echo "Removed build-output files. DONE!"
