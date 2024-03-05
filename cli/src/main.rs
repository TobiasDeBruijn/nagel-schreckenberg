use clap::Parser;
use color_eyre::Result;
use std::env::{set_var, var};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

use sim::typedef::{Position, Road, Vehicle, Velocity};

#[derive(Parser)]
pub struct Args {}

fn main() -> Result<()> {
    color_eyre::install()?;

    if var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "INFO");
    }

    registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();

    let _args = Args::parse();

    //Create road for testing the printing
    let road = make_test_road();

    road.pretty_print();

    Ok(())
}

fn make_test_road() -> Road {
    Road::new(
        vec![
            Vehicle::new(Position::new(0, 0)),
            Vehicle::new(Position::new(0, 1)),
            Vehicle::new(Position::new(5, 2)),
            Vehicle::new(Position::new(3, 2)),
            Vehicle::new(Position::new(2, 2)),
            Vehicle::new(Position::new(1, 2)),
            Vehicle::new(Position::new(100, 2)),
        ],
        vec![Velocity::new(1), Velocity::new(2)],
    )
}
