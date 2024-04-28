#!/usr/bin/env zsh

export HANNA_ROOT=${HOME}/RustProjects/hanna
export top=cfg_testbench_3
wavefile=waves.vcd
work="--work=lib_1"

ghdl remove
\rm -rf compile.sh
\rm -rf waves.vcd

../target/release/hanna \
    script \
    -l ${HANNA_ROOT}/tomls/libraries.toml \
    -t ${HANNA_ROOT}/tomls/tools/ghdl.toml \
    lib_1.${top} \
    || exit 5

source compile.sh || exit 10
# elaborate
ghdl elaborate --std=08 ${work} ${top} || exit 20
ghdl run       --std=08 ${work} ${top} --vcd=${wavefile} --ieee-asserts=disable-at-0  || exit 30
gtkwave ${wavefile} waveform.gtkw &
