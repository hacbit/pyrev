#!/bin/bash

cargo build --release

echo "Copying binary to /usr/local/bin"
sudo cp target/release/pyrev /usr/local/bin/pyrev