#!/usr/bin/env zsh

cargo flamegraph --bench "day$1" --open -- --bench
