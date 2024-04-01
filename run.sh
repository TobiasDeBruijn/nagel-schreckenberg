#!/bin/bash
set -e

run_with() {
	cargo r --release -- -s 50 -i 200 -p density --l1 $1 --l2 $2 --l3 $3 --p_decel 0.4 --p_lane_change $4 -v -o "data/probabilities/$4/$1$2$3"
}

new_run_with 5 5 5 0.8
new_run_with 4 5 5 0.8
new_run_with 3 5 5 0.8
new_run_with 2 5 5 0.8