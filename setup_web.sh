#!/bin/bash
set -eu

# Pre-requisites:
cargo install --version 0.2.84 wasm-bindgen-cli
rustup target add wasm32-unknown-unknown