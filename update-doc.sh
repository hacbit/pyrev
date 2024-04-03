#!/bin/bash

echo "Updating documentation..."
runner="cargo doc --no-deps \
    -p pyrev \
    -p pyrev_ast \
    -p pyrev_ast_derive"

echo Try run: $runner
$runner

echo "Documentation updated."

echo "Delete old docs"
rm -rf ./doc/*

echo "Copy new docs"
cp -r ./target/doc/* ./doc