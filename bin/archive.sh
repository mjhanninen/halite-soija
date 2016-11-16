#!/bin/bash
TAG=$(git describe --dirty)
test -d bots || mkdir bots
cargo build --release
cp target/release/MyBot bots/MyBot-$TAG
