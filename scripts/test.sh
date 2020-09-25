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

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} without features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --no-default-features

    /bin/echo -e "\e[0;33m***** Testing ${package} with all features *****\e[0m\n"
    cargo test --all-features --manifest-path "${package}"/Cargo.toml
}

test_package_generic_wasm32() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} without features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --no-default-features --target wasm32-unknown-unknown

    /bin/echo -e "\e[0;33m***** Testing ${package} with all features *****\e[0m\n"
    cargo test --all-features --manifest-path "${package}"/Cargo.toml
}

test_package_with_feature() {
    local package=$1
    local feature=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} with feature '${feature}' *****\e[0m\n"
    cargo test --manifest-path "$(dirname "$0")/../${package}/Cargo.toml" --features "${feature}" --no-default-features
}

test_package_generic "toph-node"
test_package_generic_wasm32 "toph-runtime"

test_package_with_feature "toph-node" "runtime-benchmarks"
test_package_with_feature "toph-runtime" "runtime-benchmarks"

