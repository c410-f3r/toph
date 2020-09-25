#!/usr/bin/env bash

set -eux

export RUST_BACKTRACE=full
export RUSTFLAGS='
    -D bad_style
    -D future_incompatible
    -D missing_debug_implementations
    -D nonstandard_style
    -D rust_2018_compatibility
    -D rust_2018_idioms
    -D unused_lifetimes
    -D unused_qualifications
    -D warnings
'

test_package_with_features() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Testing '${package}' with features '${features}' *****\e[0m\n"
    cargo test --features "${features}" --manifest-path "$(dirname "$0")/../${package}/Cargo.toml" --no-default-features
}

/bin/echo -e "\e[0;33m***** Building 'toph-runtime' without features for wasm32-unknown-unknown *****\e[0m\n"
cargo build --manifest-path "$(dirname "$0")/../toph-runtime/Cargo.toml" --no-default-features --target wasm32-unknown-unknown

test_package_with_features "toph-runtime" "std"
test_package_with_features "toph-runtime" "runtime-benchmarks,std"

test_package_with_features "toph-node" ""
test_package_with_features "toph-node" "runtime-benchmarks"


