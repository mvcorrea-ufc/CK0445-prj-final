#!/bin/bash

# Script to build and generate .bin files for ESP32-C3
# Usage: ./build-bin.sh [release|debug]

set -e

PROJECT_NAME="esp32-c3-mqtt-counter"

if [ "$1" = "debug" ]; then
    echo "Building debug version..."
    cargo build
    PROFILE="debug"
elif [ "$1" = "release" ] || [ -z "$1" ]; then
    echo "Building release version..."
    cargo build --release
    PROFILE="release"
else
    echo "Usage: $0 [release|debug]"
    echo "  release (default) - Build release version"
    echo "  debug            - Build debug version"
    exit 1
fi

ELF_PATH="target/riscv32imc-unknown-none-elf/${PROFILE}/${PROJECT_NAME}"

if [ -f "$ELF_PATH" ]; then
    echo "Generating .bin file..."
    esptool.py --chip esp32c3 elf2image --flash-mode dio --flash-freq 80m "$ELF_PATH"
    echo "Generated: ${ELF_PATH}.bin"
    ls -la "${ELF_PATH}.bin"
else
    echo "Error: ELF file not found at $ELF_PATH"
    exit 1
fi