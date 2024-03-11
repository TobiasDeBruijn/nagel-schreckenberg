use clap::Parser;
use color_eyre::Result;
use std::env::{set_var, var};
use std::thread::sleep;
use std::time::Duration;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

use sim::typedef::{Position, Road, Vehicle, Velocity};

#[derive(Parser)]
pub struct Args {
    #[clap(short)]
    #[clap(default_value = "false")]
    verbose: bool,
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

    loop {
        road = sim::step(road);
        road.pretty_print();

        sleep(Duration::from_millis(1));
    }
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
        200,
        0.3,
        vehicles,
        vec![Velocity::new(9), Velocity::new(9), Velocity::new(9)],
    )
}
