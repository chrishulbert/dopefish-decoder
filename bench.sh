#!/bin/bash
for i in {1..400}; do
    rm -rf target
    cargo build --release
done
