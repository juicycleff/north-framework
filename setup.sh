#!/bin/bash

echo "######## INSTALL RUST BINS #######"
rustup component add clippy

echo "######## INSTALL REQUIRED DEV BINS #######"
echo "Installing Binaries"
cargo install git-cliff
cargo install watch
cargo install --force cargo-make
cargo install cargo-bloat