#!/bin/bash

# remove old downloads
rm -r ./downloads

# recreate downloads folder
mkdir downloads

# remove old targets
cargo clean

# build deb installer file for Debian-based Systems
cargo deb --output ./downloads/debian

# build a windows executable
cargo build --release --target=x86_64-pc-windows-gnu --verbose --target-dir=downloads
