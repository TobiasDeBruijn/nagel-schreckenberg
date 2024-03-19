use clap::Parser;
use color_eyre::Result;
use sim::road::create_road;
use std::env::{set_var, var};
use std::time::Instant;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

use sim::typedef::{IterationInfo, Position, Road, Vehicle, Velocity};

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

    // run_iterations_with_changing_deceleration_probability(args.iterations, 1.0);
    run_iterations_with_changing_density(args.iterations, 1.0);

    // let road = create_road(100, 0.5, vec![5, 5, 5], 0.5, 0.4, true, true);
    // road.pretty_print();

    // //Create road for testing the printing
    // let mut road = make_test_road(16);
    // road.pretty_print();

    // let current_datetime = chrono::Local::now();

    // let csv_file_name = format!("output_{}.csv", current_datetime.format("%y%m%d%H%M%S"));

    // let start = Instant::now();
    // // let iteration_info = IterationInfo::new(0, start.elapsed(), road.clone());
    // // iteration_info.initialize_csv("output.csv");
    // let mut iteration = 0;
    // for _ in 0..args.iterations {
    //     iteration += 1;
    //     road = sim::step(road);
    //     let iter_info = IterationInfo::new(iteration, start.elapsed(), road.clone(), csv_file_name.as_str());
    //     iter_info.save_iteration_to_csv();
    // }

    // let end = start.elapsed();
    // road.pretty_print();

    // println!("Elapsed: {end:?}");

    Ok(())
}

fn run_iterations_with_changing_density(iterations: usize, max_density: f32) {
    let current_datetime = chrono::Local::now();
    let csv_file_name = format!("density_{}.csv", current_datetime.format("%y%m%d%H%M%S"));
    let mut iteration = 0;

    for i in float_range(0.01, 1000, max_density) {
        iteration += 1;

        let mut road = create_road(100, i, vec![9, 9, 9], 0.4, 0.5, true, true);

        let start = Instant::now();

        for _ in 0..iterations {
            road = sim::step(road);
        }

        let iter_info = IterationInfo::new(
            iteration,
            start.elapsed(),
            road.clone(),
            csv_file_name.as_str(),
        );
        iter_info.save_iteration_to_csv();

        let end = start.elapsed();

        println!("Elapsed: {end:?}");
    }
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
) {
    let current_datetime = chrono::Local::now();
    let csv_file_name = format!(
        "lane_change_{}.csv",
        current_datetime.format("%y%m%d%H%M%S")
    );
    let mut iteration = 0;
    for i in float_range(0.0, 50, max_lane_change_probability) {
        iteration += 1;
        let mut road = make_test_road(16, i, 0.3);

        let start = Instant::now();

        for _ in 0..iterations {
            road = sim::step(road);
        }

        let iter_info = IterationInfo::new(
            iteration,
            start.elapsed(),
            road.clone(),
            csv_file_name.as_str(),
        );
        iter_info.save_iteration_to_csv();

        let end = start.elapsed();

        println!("Elapsed: {end:?}");
    }
}

fn run_iterations_with_changing_deceleration_probability(
    iterations: usize,
    max_deceleration_probability: f32,
) {
    let current_datetime = chrono::Local::now();
    let csv_file_name = format!(
        "deceleration_{}.csv",
        current_datetime.format("%y%m%d%H%M%S")
    );
    let mut iteration = 0;
    for i in float_range(0.0, 1000, max_deceleration_probability) {
        iteration += 1;

        let mut road = create_road(100, 0.3, vec![9, 9, 9], i, 0.5, true, true);

        let start = Instant::now();

        for _ in 0..iterations {
            road = sim::step(road);
        }

        let iter_info = IterationInfo::new(
            iteration,
            start.elapsed(),
            road.clone(),
            csv_file_name.as_str(),
        );
        iter_info.save_iteration_to_csv();

        let end = start.elapsed();

        println!("Elapsed: {end:?}, deceleration_probability: {i}");
    }
}

fn make_test_road(
    cars_per_lane: usize,
    lane_change_probability: f32,
    deceleration_probability: f32,
) -> Road {
    //Add vehicles to the road to all three lanes
    let mut vehicles = Vec::new();
    for i in 0..cars_per_lane as u8 {
        vehicles.push(Vehicle::new(
            Position::new(i, 0),
            None,
            lane_change_probability,
            lane_change_probability,
        ));
        vehicles.push(Vehicle::new(
            Position::new(i, 1),
            None,
            lane_change_probability,
            lane_change_probability,
        ));
        vehicles.push(Vehicle::new(
            Position::new(i, 2),
            None,
            lane_change_probability,
            lane_change_probability,
        ));
    }

    Road::new(
        100,
        deceleration_probability,
        vehicles,
        vec![Velocity::new(9), Velocity::new(9), Velocity::new(9)],
    )
}
