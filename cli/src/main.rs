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

        sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn make_test_road() -> Road {
    Road::new(
        100,
        0.0,
        (0..60)
            .into_iter()
            .map(|x| Vehicle::new(Position::new(x, 0), 0.5, 0.8))
            .collect::<Vec<_>>(),
        vec![Velocity::new(3); 3],
    )
}
