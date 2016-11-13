#!/bin/bash

HALITE_DIR=${HALITE_DIR:-../Environment}
HALITE_EXE=${HALITE_EXE:-$HALITE_DIR/halite}

cargo build
test -d rec || mkdir rec
cd rec
exec ../$HALITE_EXE -d "30 30" ../target/debug/MyBot ../target/debug/RandomBot
