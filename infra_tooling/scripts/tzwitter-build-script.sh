#! /usr/bin/env bash
set -x

RPC_ENDPOINT="https://ghostnet.tezos.marigold.dev/"

# import account
account_alias=deployer
PUBLIC_KEY=$(octez-client --endpoint "$RPC_ENDPOINT" import secret key $account_alias unencrypted:${EDSK_KEY} | grep "address added" | awk '{ print $4}') ; echo $PUBLIC_KEY

# check dependencies
which smart-rollup-installer > /dev/null || (echo "smart-rollup-installer should be installed" && echo "cargo install tezos-smart-rollup-installer --git https://gitlab.com/tezos/tezos" && exit 1)
which xxd > /dev/null || (echo "xxd should be installed" && exit 1)
which wasm-strip > /dev/null || (echo "wasm-strip should be installed" && echo "https://github.com/WebAssembly/wabt" exit 1)
which ligo > /dev/null || (echo "ligo should be installed" && echo "https://ligolang.org/docs/intro/installation?lang=jsligo" exit 1)

cd /mounted_volume/repo_volume/09_tzwitter_app

# deploying the layer 1 contract
MICHELSON=$(ligo compile contract smart_contract/dummy-fa2.jsligo)
STORAGE=$(ligo compile storage smart_contract/dummy-fa2.jsligo initial_storage)

export TZWITTER_L1_CONTRACT=$(octez-client --endpoint "$RPC_ENDPOINT" originate contract tzwitter transferring 0 from $account_alias running "$MICHELSON" --init "$STORAGE" --burn-cap 1.0 --force | grep "New contract" | awk '{ print $3}') ; echo $TZWITTER_L1_CONTRACT

# Compiling the kernel
cargo build --release --target wasm32-unknown-unknown --manifest-path kernel/Cargo.toml

rm -rf rollup
mkdir rollup

# Copy the kernel in the rollup directory
cp ../target/wasm32-unknown-unknown/release/tzwitter_kernel.wasm ./rollup/kernel.wasm

# Installing the kernel
wasm-strip ./rollup/kernel.wasm

# Using the smart-rollup-installer
# It will generate the installer.hex
# And split the kernel
smart-rollup-installer get-reveal-installer --upgrade-to rollup/kernel.wasm --output rollup/installer.hex --preimages-dir rollup/wasm_2_0_0

# Setup the DAC
mkdir -p rollup/wasm_2_0_0

# Copy the kernel in the rollup directory
cp ../target/wasm32-unknown-unknown/release/tzwitter_kernel.wasm ./rollup/kernel.wasm

# saving L1 contract addr to a file
echo $TZWITTER_L1_CONTRACT > contractL1.metadata
echo $PUBLIC_KEY > deployerKey.metadata
echo "tzwitter" > projectName.metadata

# TODO: Export the entire /rollup dir into ./infra_tooling/build_artifacts and then trigger pipeline that wraps it into a Docker image.