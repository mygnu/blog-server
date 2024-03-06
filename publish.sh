#!/bin/bash
# get version from Cargo.toml
version=$(grep -oP '^\s*version = "\K[^"]*' Cargo.toml | head -n 1)

echo "Publishing version ${version}"

# build and push the image
cargo build --release

image="registry.gill.desi/blog-server:${version}"

docker build --tag="${image}" .
docker push "${image}"
