#!/bin/sh
cargo run --manifest-path ../scripts/script_enum/Cargo.toml > src/scripts.rs
cargo fmt
