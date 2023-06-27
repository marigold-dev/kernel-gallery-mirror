#! /usr/bin/env bash
set -x

# check dependencies
which smart-rollup-installer > /dev/null || (echo "smart-rollup-installer should be installed" && echo "cargo install tezos-smart-rollup-installer --git https://gitlab.com/tezos/tezos" && exit 1)
which xxd > /dev/null || (echo "xxd should be installed" && exit 1)
which wasm-strip > /dev/null || (echo "wasm-strip should be installed" && echo "https://github.com/WebAssembly/wabt" exit 1)

cd /mounted_volume/repo_volume/

# Compiling the kernel
cargo build --release --target wasm32-unknown-unknown --manifest-path kernel/Cargo.toml

rm -rf rollup

# Copy the kernel in the rollup directory
mkdir -p rollup
cp ../target/wasm32-unknown-unknown/release/kernel.wasm ./rollup/kernel.wasm

# Installing the kernel
wasm-strip ./rollup/kernel.wasm

# Using the smart-rollup-installer
# It will generate the installer.hex
# And split the kernel
smart-rollup-installer get-reveal-installer --upgrade-to rollup/kernel.wasm --output rollup/installer.hex --preimages-dir rollup/wasm_2_0_0

# TODO: Export the entire /rollup dir into ./infra_tooling/build_artifacts and then trigger pipeline that wraps it into a Docker image.