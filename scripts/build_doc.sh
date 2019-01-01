#!/bin/bash

set -ex

function run_rustdoc()
{
  cd build/yalr && \
    ./../../scripts/rustdoc_wrapper.py "$1" && \
    cd ../..
}

export CARGO_PKG_VERSION=unknown
run_rustdoc yalr
run_rustdoc yalr_proc_macro
run_rustdoc yalr_cli
run_rustdoc yalr_codegen
run_rustdoc yalr_core

mkdir -p static/doc/develop
cp -r build/yalr/target/doc/* static/doc/develop
find build/yalr/target/doc -maxdepth 0 -type f -exec sh -c 'cp {} "static/doc/develop/$(basename {})"' \;
