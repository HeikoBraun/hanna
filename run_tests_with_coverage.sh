#!/usr/bin/env sh

export HANNA_ROOT=$PWD
export RUST_LOG=trace
cargo llvm-cov --html 
firefox -new-window ${HANNA_ROOT}/target/llvm-cov/html/index.html
