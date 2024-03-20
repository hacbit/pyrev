#!/bin/bash

cargo build --release

sudo cp target/release/pyrev /usr/local/bin/pyrev