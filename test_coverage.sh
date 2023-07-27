#!/bin/bash
rm -rf coverage
mkdir coverage
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test --all-features
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --keep-only 'src/*' --keep-only 'tests/*' -o coverage/html
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --keep-only 'src/*' --keep-only 'tests/*' -o coverage/tests.lcov