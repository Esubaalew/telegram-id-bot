#!/bin/bash

echo "Building Telegram ID Bot for Vercel..."

if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi


echo "Compiling Rust binary..."
cargo build --release

if [ $? -eq 0 ]; then
    echo " Build successful!"
    echo "Binary location: target/release/telegram-id"
    ls -la target/release/telegram-id
else
    echo " Build failed!"
    exit 1
fi

echo " Ready for deployment!"