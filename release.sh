#!/bin/bash

# extract version from Cargo.toml
version=$(grep -m 1 '^version' Cargo.toml | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')
echo "Program version: $version"

# Run cargo build with the --release flag
cargo build --release
cd target/release/

app_name="writeme"

# {REPO_NAME}-{VERSION}-{OPERATING_SYSTEM}-{ARCHITECTURE}.tar.gz
target_darwin_arm64="$app_name-$version-darwin-arm64.tar.gz"

# Create the archive
tar -czf "$target_darwin_arm64" "$app_name"
echo "Created $target_darwin_arm64"