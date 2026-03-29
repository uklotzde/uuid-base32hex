# SPDX-FileCopyrightText: The uuid-base32hex authors
# SPDX-License-Identifier: CC0-1.0

# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

[doc('Format source code')]
fmt:
    cargo fmt --all

[doc('Run clippy with various feature combinations')]
clippy:
    cargo clippy --locked --workspace --all-targets --no-default-features
    cargo clippy --locked --workspace --all-targets --no-default-features --features all-controllers
    cargo clippy --locked --workspace --no-deps --all-targets --all-features -- -D warnings --cap-lints warn

[doc('Check build of all feature combinations')]
check-features:
    cargo hack check --each-feature --no-dev-deps

[doc('Run cargo check for the WASM target with default features enabled')]
check-wasm:
    cargo check --locked --workspace --target wasm32-unknown-unknown --features js

[doc('Run unit tests')]
test:
    RUST_BACKTRACE=1 cargo test --locked --all-features -- --nocapture

[doc('Set up (and update) tooling')]
setup:
    # Ignore rustup failures, because not everyone might use it
    rustup self update || true
    # cargo-edit is needed for `cargo upgrade`
    cargo install cargo-edit cargo-hack just
    pip install -U pre-commit
    #pre-commit install --hook-type commit-msg --hook-type pre-commit

[doc('Upgrade (and update) dependencies')]
upgrade: setup
    pre-commit autoupdate
    cargo upgrade --incompatible --pinned
    cargo update

[doc('Run pre-commit hooks')]
pre-commit:
    pre-commit run --all-files
