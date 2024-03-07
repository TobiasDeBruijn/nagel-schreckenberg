use clap::Parser;
use color_eyre::Result;
use std::env::{set_var, var};
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
    }
}

fn make_test_road() -> Road {
    Road::new(
        100,
        0.0,
        (0..30)
            .into_iter()
            .map(|x| Vehicle::new(Position::new(x, 0), 0.9, 0.1))
            .collect::<Vec<_>>(),
        vec![Velocity::new(10), Velocity::new(30), Velocity::new(30)],
    )
}
