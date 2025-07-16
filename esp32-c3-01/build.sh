#!/bin/bash

# Script to build and generate .bin file
# Usage: ./build.sh [release]

set -e

PROJECT_NAME="esp32-c3-mqtt-counter"

if [ "$1" = "release" ]; then
    echo "Building release version..."
    cargo build --release
    PROFILE="release"
else
    echo "Building debug version..."
    cargo build
    PROFILE="debug"
fi

ELF_PATH="target/riscv32imc-unknown-none-elf/${PROFILE}/${PROJECT_NAME}"
BIN_PATH="target/${PROJECT_NAME}-${PROFILE}.bin"

if [ -f "$ELF_PATH" ]; then
    echo "Generating .bin file..."
    espflash save-image --chip esp32c3 "$ELF_PATH" "$BIN_PATH"
    echo "Generated: $BIN_PATH"
    ls -la "$BIN_PATH"
else
    echo "Error: ELF file not found at $ELF_PATH"
    exit 1
fi