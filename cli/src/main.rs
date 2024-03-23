use clap::Parser;
use color_eyre::Result;
use sim::road::create_road;
use std::env::{set_var, var};
use std::time::Instant;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

use sim::typedef::{IterationInfo, IterationWriter};

#[derive(Parser)]
pub struct Args {
    #[clap(short)]
    #[clap(default_value = "false")]
    verbose: bool,
    #[clap(short)]
    #[clap(default_value = "100")]
    iterations: usize,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    if var("RUST_LOG").is_err() {
        if args.verbose {
            set_var("RUST_LOG", "TRACE");
        } else {
            set_var("RUST_LOG", "INFO");
        }
    }

    registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();

    // run_iterations_with_changing_density(args.iterations, 0.5);
    // run_iterations_with_changing_lane_change_probability(args.iterations, 0.5);

    let start = Instant::now();

    let current_datetime = chrono::Local::now();
    let csv_file_name = format!("density_{}.csv", current_datetime.format("%y%m%d%H%M%S"));
    let iteration_writer = IterationWriter::new(csv_file_name.as_str());

    let iteration_infos = run_iterations_with_changing_density(args.iterations, 0.5);

    iteration_writer.write_iteration_infos_to_csv(&iteration_infos);

    let current_datetime = chrono::Local::now();
    let csv_file_name = format!(
        "lane_change_{}.csv",
        current_datetime.format("%y%m%d%H%M%S")
    );
    let iteration_writer = IterationWriter::new(csv_file_name.as_str());

    let iteration_infos =
        run_iterations_with_changing_lane_change_probability(args.iterations, 0.5);

    iteration_writer.write_iteration_infos_to_csv(&iteration_infos);

    let current_datetime = chrono::Local::now();
    let csv_file_name = format!(
        "deceleration_{}.csv",
        current_datetime.format("%y%m%d%H%M%S")
    );
    let iteration_writer = IterationWriter::new(csv_file_name.as_str());
    let iteration_infos =
        run_iterations_with_changing_deceleration_probability(args.iterations, 0.5);
    iteration_writer.write_iteration_infos_to_csv(&iteration_infos);

    let end = start.elapsed();

    println!("Total Time Elapsed: {end:?}");

    Ok(())
}

fn run_iterations_with_changing_density(iterations: usize, max_density: f32) -> Vec<IterationInfo> {
    let mut iteration = 0;

    let mut iteration_infos = Vec::new();

    for i in float_range(0.01, 1000, max_density) {
        iteration += 1;

        let mut road = create_road(100, i, vec![5, 5, 5], 0.4, 0.5, true, true);

        let start = Instant::now();

        for _ in 0..iterations {
            road = sim::step(road);
        }

        let iteration_info = IterationInfo::new(iteration, start.elapsed(), road.clone());

        iteration_infos.push(iteration_info);
        //let end = start.elapsed();

        //println!("Iteration {iteration:?}, Time Elapsed: {end:?}");
    }

    iteration_infos
}

fn float_range(start: f32, iter: usize, end: f32) -> Vec<f32> {
    let mut range = Vec::new();
    let step = (end - start) / iter as f32;
    for i in 0..iter {
        range.push(start + step * i as f32);
    }
    range
}

fn run_iterations_with_changing_lane_change_probability(
    iterations: usize,
    max_lane_change_probability: f32,
) -> Vec<IterationInfo> {
    let mut iteration_infos = Vec::new();

    let mut iteration = 0;
    for i in float_range(0.0, 1000, max_lane_change_probability) {
        iteration += 1;
        let mut road = create_road(100, 0.3, vec![5, 5, 5], 0.4, i, true, true);

        let start = Instant::now();

        for _ in 0..iterations {
            road = sim::step(road);
        }

        let iteration_info = IterationInfo::new(iteration, start.elapsed(), road.clone());

        iteration_infos.push(iteration_info);
        // let end = start.elapsed();

        // println!("Iteration {iteration:?}, Time Elapsed: {end:?}");
    }

    iteration_infos
}

fn run_iterations_with_changing_deceleration_probability(
    iterations: usize,
    max_deceleration_probability: f32,
) -> Vec<IterationInfo> {
    let mut iteration = 0;

    let mut iteration_infos = Vec::new();

    for i in float_range(0.0, 1000, max_deceleration_probability) {
        iteration += 1;

        let mut road = create_road(100, 0.3, vec![5, 5, 5], i, 0.5, true, true);

        let start = Instant::now();

        for _ in 0..iterations {
            road = sim::step(road);
        }

        let iteration_info = IterationInfo::new(iteration, start.elapsed(), road.clone());

        iteration_infos.push(iteration_info);

        // let end = start.elapsed();

        // println!("Iteration {iteration:?}, Time Elapsed: {end:?}");
    }

    iteration_infos
}
