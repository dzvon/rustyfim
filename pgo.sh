#!/bin/bash

rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo run --release --target $1 -p bench --features=simd -- -m 0.4 ./tests/data/chess.dat

llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

maturin build --release --target $1 --features simd -o dist -i python3.10 -- -Cprofile-use=/tmp/pgo-data/merged.profdata
