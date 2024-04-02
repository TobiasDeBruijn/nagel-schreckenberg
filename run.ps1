# Define a function to run the cargo command with specified parameters
function run_with {
    param(
        [int]$sim_count,
        [int]$iteration_count,
        [int]$l1,
        [int]$l2,
        [int]$l3,
        [double]$p_decel,
        [double]$p_lane_change
    )

    cargo r --release -- -s $sim_count -i $iteration_count --parameter-under-test density --l1 $l1 --l2 $l2 --l3 $l3 --p-decel $p_decel --p-lane-change $p_lane_change -v -o "probabilities/decel/$p_decel/$l1$l2$l3"
}

function run {
    param(
        [double]$p_decel
    )
    
    run_with 50 200 5 5 5 $p_decel 1.0

    run_with 50 200 3 5 5 $p_decel 1.0
    run_with 50 200 5 3 5 $p_decel 1.0

    run_with 50 200 4 5 5 $p_decel 1.0
    run_with 50 200 5 4 5 $p_decel 1.0

    run_with 50 200 3 4 5 $p_decel 1.0
    run_with 50 200 4 3 5 $p_decel 1.0

}

function run_for_simulations_evaluation {

    run_for_simulation 1
    run_for_simulation 10
    run_for_simulation 50
    run_for_simulation 100
}

function run_for_simulation {
    param(
        [int]$sim_count
    )
    cargo r --release -- -s $sim_count -i 200 --parameter-under-test density --l1 5 --l2 5 --l3 5 --p-decel 0.4 --p-lane-change 1.0 -v -o "sims-evaluation/$($sim_count)sim"
}

# Run the cargo command with different parameters
# run 0.2
# run 0.4
# run 0.6
run 0.8

# run_for_simulations_evaluation