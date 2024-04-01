#!/bin/bash
set -e

run_with() {
	cargo r --release -- -s 50 -i 200 --parameter-under-test density --l1 $1 --l2 $2 --l3 $3 --p-decel 0.4 --p-lane-change $4 -v -o "probabilities/$4/$1$2$3"
}

run_with 5 5 5 0.4
run_with 4 5 5 0.4
run_with 3 5 5 0.4
run_with 2 5 5 0.4
run_with 5 5 5 0.6
run_with 4 5 5 0.6
run_with 3 5 5 0.6
run_with 2 5 5 0.6