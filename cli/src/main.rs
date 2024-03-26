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
    #[clap(default_value = "density")]
    parameter_under_test: ParameterUnderTest,
    #[clap(short)]
    #[clap(default_value = "100")]
    road_len: u8,
    #[clap(long)]
    l1: u8,
    #[clap(long)]
    l2: u8,
    #[clap(long)]
    l3: u8,
    #[clap(long)]
    #[clap(default_value = "false")]
    pretty_print: bool,
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
    let fmt = now.format("%Y-%m-%d_%H%M").to_string();

    let sim_type = match args.parameter_under_test {
        ParameterUnderTest::Density => SimulationType::Density(0.01, 0.5, 0.08),
        ParameterUnderTest::PDecel => SimulationType::Deceleration(0.01, 1.0, 0.001),
        ParameterUnderTest::PLaneChange => SimulationType::LaneChange(0.01, 1.0, 0.001),
    };

    let file_name = match args.parameter_under_test {
        ParameterUnderTest::Density => format!("density_{fmt}.csv"),
        ParameterUnderTest::PDecel => format!("p_decel_{fmt}.csv"),
        ParameterUnderTest::PLaneChange => format!("p_lane_change_{fmt}.csv"),
    };

    let simulation_handler = SimulationsHandler::new(
        args.simulations,
        args.iterations,
        sim_type.clone(),
        SimulationWriter::new(&file_name),
        args.verbose,
        vec![args.l1, args.l2, args.l3],
        args.pretty_print,
    );

    // Construct the MetaData
    let metadata = sim::typedef::MetaData {
        road_len: args.road_len,
        num_simulations: args.simulations,
        iterations_per_simulation: args.iterations,
        sim_type,
        speeds_per_lane: vec![args.l1, args.l2, args.l3]
    };

    let iteration_infos = simulation_handler.run_simulations();
    simulation_handler.save_simulation_results(&iteration_infos, &metadata);
    Ok(())
}
