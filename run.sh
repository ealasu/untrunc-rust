#!/bin/bash
set -e

cargo build --release --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/release/untrunc bbb:~/
ssh -t bbb sudo /home/debian/run.sh
