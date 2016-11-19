#!/bin/bash

HALITE_DIR=${HALITE_DIR:-../Environment}
HALITE_EXE=${HALITE_EXE:-$HALITE_DIR/halite}

BOT_1=../$(bin/curry.sh ../target/debug/MyBot)
BOT_2=../$(bin/curry.sh ../target/debug/MyBot)

cargo build
test -d rec || mkdir rec
cd rec
exec ../$HALITE_EXE -d "30 30" $BOT_1 $BOT_2
