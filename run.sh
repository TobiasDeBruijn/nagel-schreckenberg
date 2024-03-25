#!/bin/bash
set -e

rm -rf data/*

run_with() {
	cargo r --release -- -s 50 -i 200  -p density --l1 $1 --l2 $2 --l3 $3 -v
	mv data/density_2024*.csv data/density_$1_$2_$3_0.8.csv
	mv data/density_2024*.metadata data/density_$1_$2_$3_0.8.metadata
}

run_with 3 3 3
run_with 3 4 5
run_with 3 4 4
run_with 3 5 5
run_with 5 5 5

