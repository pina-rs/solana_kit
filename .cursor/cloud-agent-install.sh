#!/usr/bin/env bash
# Install the toolchain pinned by rust-toolchain.toml and fetch the locked
# workspace crates. Idempotent: a second run only confirms the toolchain and
# refreshes the Cargo registry. Long-running processes belong in start, not here.
set -euo pipefail

if [ -d /agent/repos/solana_kit ]; then
  cd /agent/repos/solana_kit
fi

if ! command -v cc >/dev/null 2>&1 || ! command -v g++ >/dev/null 2>&1 || ! command -v pkg-config >/dev/null 2>&1; then
  sudo apt-get update
  sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential ca-certificates curl g++ pkg-config
fi

# The default image points `cc` and `c++` at clang. cc-rs then passes
# `--target`, and that clang cannot see the libstdc++ headers. rust-lld also
# cannot find `-lstdc++`. `cargo test --all-features` fails while building the
# LeptosKit C++ runtime. Prefer gcc and g++ on the default PATH.
sudo ln -sfn "$(command -v gcc)" /usr/local/bin/cc
sudo ln -sfn "$(command -v g++)" /usr/local/bin/c++

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain none --profile minimal --no-modify-path
  sudo ln -sfn "${HOME}/.cargo/bin/rustup" /usr/local/bin/rustup
  sudo ln -sfn "${HOME}/.cargo/bin/cargo" /usr/local/bin/cargo
  sudo ln -sfn "${HOME}/.cargo/bin/rustc" /usr/local/bin/rustc
fi

# rust-toolchain.toml selects the channel, clippy, rustfmt, and Wasm targets.
# `rustup show` installs that toolchain when it is missing.
rustup show
rustc --version
cargo --version

cargo fetch --locked
