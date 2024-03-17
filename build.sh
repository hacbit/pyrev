#!/bin/bash

cargo build --release

cp target/release/pyrev ~/.cargo/bin/pyrev