# Define a function to run the cargo command with specified parameters
function run_with {
    param(
        [int]$l1,
        [int]$l2,
        [int]$l3,
        [double]$p_lane_change
    )

    cargo r --release -- -s 50 -i 200 --parameter-under-test density --l1 $l1 --l2 $l2 --l3 $l3 --p-decel 0.4 --p-lane-change $p_lane_change -v -o "probabilities/$p_lane_change/$l1$l2$l3"
}

# Run the cargo command with different parameters
run_with 5 5 5 0.4
run_with 4 5 5 0.4
run_with 3 5 5 0.4
run_with 2 5 5 0.4
run_with 5 5 5 0.6
run_with 4 5 5 0.6
run_with 3 5 5 0.6
run_with 2 5 5 0.6