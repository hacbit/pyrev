#!/bin/bash

echo "Updating documentation..."
runner="cargo doc --no-deps -p pyrev"

for dep in src/pyrev*
do
    if [ -d $dep ]; then
        runner="$runner -p $(basename $dep)"
    fi
done

echo Try run: $runner
$runner

echo "Documentation updated."

echo "Delete old docs"
rm -rf ./doc/*

echo "Copy new docs"
cp -r ./target/doc/* ./doc