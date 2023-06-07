#!/usr/bin/env bash

VER=0.0.4

# linux deps:
# rustup target add x86_64-unknown-linux-musl
echo 'generating for linux...'
cargo build --release --target x86_64-unknown-linux-musl
mv -f target/x86_64-unknown-linux-musl/release/rmig dist/rmig-$VER-linux

# windows deps:
# rustup target add x86_64-pc-windows-gnu
# apt install -y mingw-w64
echo 'generating for windows...'
RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" \
    cargo build --release --target x86_64-pc-windows-gnu
mv -f target/x86_64-pc-windows-gnu/release/rmig.exe dist/rmig-$VER-windows.exe
