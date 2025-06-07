#!/bin/bash

echo "🦀 Building Ultra-Fast Taproot Vanity Generator..."
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust from: https://rustup.rs/"
    echo
    echo "After installation, restart your terminal and run this script again."
    exit 1
fi

echo "✅ Rust found, building optimized release..."
cargo build --release

if [ $? -eq 0 ]; then
    echo
    echo "🎉 Build successful!"
    echo
    echo "📁 Executable location: target/release/taproot-vanity"
    echo
    echo "🚀 Usage examples:"
    echo "  ./target/release/taproot-vanity"
    echo "  ./target/release/taproot-vanity --min-kek-count 3"
    echo "  ./target/release/taproot-vanity --workers 16"
    echo "  ./target/release/taproot-vanity --output-file my_kek_addresses.txt"
    echo "  ./target/release/taproot-vanity --case-sensitive"
    echo
    echo "💡 For maximum performance, use all CPU cores:"
    echo "  ./target/release/taproot-vanity --workers $(nproc)"
    echo
else
    echo "❌ Build failed!"
    echo "Check the error messages above."
    exit 1
fi
