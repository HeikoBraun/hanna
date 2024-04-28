#!/usr/bin/env sh

export HANNA_ROOT=$PWD
cargo llvm-cov --html 
firefox -new-window ${HANNA_ROOT}/target/llvm-cov/html/index.html
