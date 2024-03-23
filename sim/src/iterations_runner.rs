use std::{env::Args, time::Instant};

use crate::{
    road::create_road,
    step,
    typedef::{IterationInfo, IterationRunnerType, IterationWriter, Road},
};

pub fn run_iterations_and_save_to_csv(
    iterations: usize,
    runner_type: IterationRunnerType,
    file_path: &str,
) {
    let iteration_infos = run_iterations(iterations, runner_type);

    let iteration_writer = IterationWriter::new(file_path);

    iteration_writer.write_iteration_infos_to_csv(&iteration_infos);
}

pub fn run_iterations(iterations: usize, runner_type: IterationRunnerType) -> Vec<IterationInfo> {
    let mut iteration = 0;
    let mut iteration_infos = Vec::new();

    for i in float_range_iter(0.0, 1000, 0.5) {
        iteration += 1;

        let road = match runner_type {
            IterationRunnerType::Density(start, end, step) => {
                create_road(100, i, vec![5, 5, 5], 0.4, 0.4, true, true)
            }
            IterationRunnerType::Deceleration(start, end, step) => {
                create_road(100, 0.3, vec![5, 5, 5], i, 0.4, true, true)
            }
            IterationRunnerType::LaneChange(start, end, step) => {
                create_road(100, 0.3, vec![5, 5, 5], 0.4, i, true, true)
            }
        };

        let iteration_info = run_iteration(iteration, road);

        iteration_infos.push(iteration_info);
    }

    iteration_infos
}

fn run_iteration(iteration: usize, road: Road) -> IterationInfo {
    let start = Instant::now();

    let road = step(road);

    let iteration_info = IterationInfo::new(iteration, start.elapsed(), road.clone());

    iteration_info
}

fn float_range_iter(start: f32, iter: usize, end: f32) -> Vec<f32> {
    let step = (end - start) / iter as f32;

    (0..iter)
        .into_iter()
        .map(|i| start + step * i as f32)
        .collect()
}

fn float_range_step(start: f32, end: f32, step: f32) -> Vec<f32> {
    let mut range = Vec::new();
    let mut i = start;
    while i < end {
        range.push(i);
        i += step;
    }
    range
}
