use std::env::{set_var, var};

use clap::{Parser, ValueEnum};
use color_eyre::Result;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

use sim::typedef::{SimulationType, SimulationWriter, SimulationsHandler};

#[derive(Parser)]
pub struct Args {
    #[clap(short)]
    #[clap(default_value = "false")]
    verbose: bool,
    /// How often one set of parameters should be simulated.
    /// The results are then averaged together.
    #[clap(short)]
    #[clap(default_value = "50")]
    simulations: usize,
    /// How many steps should be executed within one simulation.
    /// Every step applies the rules of the model once.
    #[clap(short)]
    #[clap(default_value = "100")]
    iterations: usize,
    #[clap(long, short, value_enum)]
    parameter_under_test: ParameterUnderTest,
}

#[derive(ValueEnum, Clone)]
enum ParameterUnderTest {
    Density,
    PDecel,
    PLaneChange,
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

    let now = chrono::Utc::now();
    let fmt = now.format("%Y-%m-%d_%H:%M").to_string();

    let simulation_handler = match args.parameter_under_test {
        ParameterUnderTest::Density => SimulationsHandler::new(
            args.simulations,
            args.iterations,
            SimulationType::Density(0.01, 0.5, 0.001),
            SimulationWriter::new(&format!("density_{fmt}.csv")),
            args.verbose,
        ),
        ParameterUnderTest::PDecel => SimulationsHandler::new(
            args.simulations,
            args.iterations,
            SimulationType::Deceleration(0.01, 1.0, 0.001),
            SimulationWriter::new(&format!("p_decel_{fmt}.csv")),
            args.verbose,
        ),
        ParameterUnderTest::PLaneChange => SimulationsHandler::new(
            args.simulations,
            args.iterations,
            SimulationType::LaneChange(0.01, 1.0, 0.001),
            SimulationWriter::new(&format!("p_lane_change_{fmt}.csv")),
            args.verbose,
        ),
    };

    let iteration_infos = simulation_handler.run_simulations();
    simulation_handler.write_simulation_results_to_csv(&iteration_infos);
    Ok(())
}
