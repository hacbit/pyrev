#!/bin/bash

is_release=0
files=

for arg in "$@"
do
    if [ "$arg" == "--release" ]; then
        is_release=1
    fi
done

# find all the .txt files in test/
for file in test/*.txt
do
    files="$files --file $file"
done

echo "Running tests with the following files: $files"

if [ "$is_release" == "1" ]; then
    echo "Running tests in release mode"
    #cargo run --release -- 
else
    echo "Running tests in debug mode"
    #cargo run --
fi

echo "Press any key to continue..."

read -n 1 -s