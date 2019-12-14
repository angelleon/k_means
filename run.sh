#!/bin/bash

cargo build --release
./target/release/k_means
sleep 0.1
cp ./points.json ~/Dev/processing/k_means/
/home/angel/Software/processing-3.5.3/processing-java --sketch=$HOME/Dev/processing/k_means --run
