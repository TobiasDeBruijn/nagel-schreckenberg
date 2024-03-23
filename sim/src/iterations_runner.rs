use std::time::Instant;

use crate::{
    step,
    typedef::{IterationInfo, Road},
};

fn float_range_iter(start: f32, iter: usize, end: f32) -> Vec<f32> {
    let step = (end - start) / iter as f32;

    (0..iter)
        .into_iter()
        .map(|i| start + step * i as f32)
        .collect()
}

pub fn float_range_step(start: f32, end: f32, step: f32) -> Vec<f32> {
    let mut range = Vec::new();
    let mut i = start;
    while i < end {
        range.push(i);
        i += step;
    }
    range
}

pub fn run_iterations(sim_nr : usize, iterations: usize, mut road : Road) -> IterationInfo {
    let start = Instant::now();

    for _ in 0..iterations {
        road = step(road);
    }

    IterationInfo::new(sim_nr, start.elapsed(), road)
}