use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::{
    step,
    typedef::{IterationInfo, Road},
};

pub fn run_iterations(sim_nr: usize, iterations: usize, mut road: Road, pretty_print: bool) -> IterationInfo {
    let start = Instant::now();
    for _ in 0..iterations {
        road = step(road);

        if pretty_print {
            road.pretty_print();
            sleep(Duration::from_millis(150));
        }
    }

    IterationInfo::new(sim_nr, start.elapsed(), road)
}
