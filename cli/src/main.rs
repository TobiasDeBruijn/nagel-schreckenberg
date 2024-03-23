use clap::Parser;
use color_eyre::Result;
use std::env::{set_var, var};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

use sim::typedef::{SimulationsHandler, SimulationType, SimulationWriter};

#[derive(Parser)]
pub struct Args {
    #[clap(short)]
    #[clap(default_value = "false")]
    verbose: bool,
    #[clap(short)]
    #[clap(default_value = "50")]
    simulations: usize,
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

    let current_datetime = chrono::Local::now();
    let csv_file_name = format!("density_{}.csv", current_datetime.format("%y%m%d%H%M%S"));

    let simulation_handler = SimulationsHandler::new(
        args.simulations,
        args.iterations,
        SimulationType::Density(0.01, 0.5, 0.001),
        SimulationWriter::new(&csv_file_name),
        args.verbose,
    );

    let iteration_infos = simulation_handler.run_simulations();

    simulation_handler.write_simulation_results_to_csv(&iteration_infos);    
    Ok(())
}