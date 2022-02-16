#!/usr/bin/env bash

set -eo pipefail

# Generate Rust files from Protobuf
mkdir tmp_proto_output
proto_dirs=$(find ./proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
for dir in $proto_dirs; do
  buf protoc \
    -I "proto" \
    -I "third_party/proto" \
    --rust_out=./tmp_proto_output \
    $(find "${dir}" -maxdepth 1 -name '*.proto')
done

# Move output Rust files to the right place
output_files=$(find ./tmp_proto_output -name '*.rs')
for file in $output_files; do
  mv $file ./contract/src
done
rm -rf tmp_proto_output
