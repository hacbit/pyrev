#!/bin/bash

is_release=0

for arg in "$@"
do
    if [ "$arg" == "--release" ]; then
        is_release=1
    fi
done

if [ "$is_release" == "1" ]; then
    echo "Building in release mode"
    cargo build --release
else
    echo "Building in debug mode"
    cargo build
fi

# find all the .txt files in test/
for file in test/*.txt
do
    echo "Running $file"
    if [ "$is_release" == "1" ]; then
        cargo run --release -- --file $file
    else
        cargo run -- --file $file
    fi
done
