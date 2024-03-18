use clap::Parser;
use color_eyre::Result;
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

    //Create road for testing the printing
    let mut road = make_test_road();
    road.pretty_print();

    let start = Instant::now();
    // let iteration_info = IterationInfo::new(0, start.elapsed(), road.clone());
    // iteration_info.initialize_csv("output.csv");
    let mut iteration = 0;
    for _ in 0..args.iterations {
        iteration += 1;
        road = sim::step(road);
        let iter_info = IterationInfo::new(iteration, start.elapsed(), road.clone());
        iter_info.save_iteration_to_csv("output.csv");
    }

    let end = start.elapsed();
    road.pretty_print();

    println!("Elapsed: {end:?}");

    Ok(())
}

fn make_test_road() -> Road {
    //Add vehicles to the road to all three lanes
    let mut vehicles = Vec::new();
    for i in 0..16 {
        vehicles.push(Vehicle::new(Position::new(i, 0), 0.5, 0.5));
        vehicles.push(Vehicle::new(Position::new(i, 1), 0.5, 0.5));
        vehicles.push(Vehicle::new(Position::new(i, 2), 0.5, 0.5));
    }

    Road::new(
        100,
        0.3,
        vehicles,
        vec![Velocity::new(9), Velocity::new(9), Velocity::new(9)],
    )
}
