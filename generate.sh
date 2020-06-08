#!/usr/bin/env bash

VER=0.0.3

# linux deps:
# rustup target add x86_64-pc-windows-musl
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

# macos deps:
# rustup target add x86_64-apple-darwin
# apt install -y clang gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libgmp-dev libxml2-dev
# git clone https://github.com/tpoechtrager/osxcross
# cd osxcross
# wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
# mv MacOSX10.10.sdk.tar.xz tarballs/
# UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
echo 'generating for macos...'
PATH="$HOME/osxcross/target/bin:$PATH" \
CC=o64-clang \
CXX=o64-clang++ \
LIBZ_SYS_STATIC=1 \
RUSTFLAGS="-C linker=x86_64-apple-darwin14-clang" \
    cargo build --release --target x86_64-apple-darwin
mv -f target/x86_64-apple-darwin/release/rmig dist/rmig-$VER-macos
