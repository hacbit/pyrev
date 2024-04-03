#!/bin/bash

echo "This script will delete all .txt files in test/"
echo "And get the new python bytecode from python files in test/ and save them as .txt files"

for file in test/*.txt
do
    echo "Deleting $file"
    rm $file
done

for file in test/*.py
do
    echo "Try run: python3 $file > ${file%.py}.txt"
    python3 $file > ${file%.py}.txt
done