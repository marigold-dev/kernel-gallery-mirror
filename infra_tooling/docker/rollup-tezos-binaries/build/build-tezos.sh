#! /usr/bin/env bash
set -x
set -euo pipefail

version="$1"
profile="$2"
repository="$3"
# build="$4"

git clone -b $version --single-branch $repository
cd tezos
# git checkout $version

export OPAMYES="true"
# Disable usage of instructions from the ADX extension to avoid incompatibility
# with old CPUs, see https://gitlab.com/dannywillems/ocaml-bls12-381/-/merge_requests/135/
export BLST_PORTABLE="yes"
wget https://sh.rustup.rs/rustup-init.sh
chmod +x rustup-init.sh
./rustup-init.sh --profile minimal --default-toolchain 1.64.0 -y
source "$HOME/.cargo/env"

# fix bwrap
chmod u+s /usr/bin/bwrap
chmod a+s /usr/bin/bwrap
chmod 777 /usr/bin/bwrap

opam init --bare --disable-sandboxing
make build-deps
eval "$(opam env)" && PROFILE="$profile" make
pwd
ls -last .
chmod +x octez-*
# cp octez-* /bin/ && ls -last /bin/
cp octez-client octez-smart-rollup-node-PtNairob octez-smart-rollup-node-alpha /bin/
rm -rf /tezos/_opam /tezos/src /tezos/.git
rm -rf /tezos/_build
rm -rf /tezos/octez-*
ls -last .
