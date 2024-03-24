use std::time::Instant;

use crate::{
    step,
    typedef::{IterationInfo, Road},
};

pub fn run_iterations(sim_nr: usize, iterations: usize, mut road: Road) -> IterationInfo {
    let start = Instant::now();
    for _ in 0..iterations {
        road = step(road);
    }

    IterationInfo::new(sim_nr, start.elapsed(), road)
}
