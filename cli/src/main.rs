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
    #[clap(default_value = "5")]
    l1: u8,
    #[clap(long)]
    #[clap(default_value = "5")]
    l2: u8,
    #[clap(long)]
    #[clap(default_value = "5")]
    l3: u8,
    #[clap(long, short)]
    #[clap(default_value = "")]
    output_name: String,
    #[clap(short, long)]
    #[clap(default_value = "0.4")]
    p_decel: f32,
    #[clap(short, long)]
    #[clap(default_value = "0.8")]
    p_lane_change: f32,
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

    println!("Path: {}", args.output_name);

    registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();

    let now = chrono::Utc::now();
    let fmt = now.format("%Y-%m-%d_%H%M").to_string();

    let sim_type = match args.parameter_under_test {
        ParameterUnderTest::Density => SimulationType::Density(0.01, 0.5, 0.003333333),
        ParameterUnderTest::PDecel => SimulationType::Deceleration(0.01, 1.0, 0.001),
        ParameterUnderTest::PLaneChange => SimulationType::LaneChange(0.01, 1.0, 0.001),
    };

    let file_name = if args.output_name != "" {
        let output_name = args.output_name;
        format!("{output_name}.csv")
    } else {
        match args.parameter_under_test {
            ParameterUnderTest::Density => format!("density_{fmt}.csv"),
            ParameterUnderTest::PDecel => format!("p_decel_{fmt}.csv"),
            ParameterUnderTest::PLaneChange => format!("p_lane_change_{fmt}.csv"),
        }
    };

    //Check if the output file already exists or if the path is invalid
    if std::path::Path::new(&file_name).exists() {
        eprintln!("Output file already exists. Please choose a different name.");
        std::process::exit(1);
    }

    let simulation_handler = SimulationsHandler::new(
        args.simulations,
        args.iterations,
        args.p_decel,
        args.p_lane_change,
        sim_type.clone(),
        SimulationWriter::new(&file_name),
        args.verbose,
        vec![args.l1, args.l2, args.l3],
    );

    // Construct the MetaData
    let metadata = sim::typedef::MetaData {
        road_len: args.road_len,
        num_simulations: args.simulations,
        iterations_per_simulation: args.iterations,
        sim_type,
        speeds_per_lane: vec![args.l1, args.l2, args.l3],
    };

    let start = std::time::Instant::now();

    let iteration_infos = simulation_handler.run_simulations();

    let duration = start.elapsed();

    simulation_handler.save_simulation_results(&iteration_infos, &metadata);

    if args.verbose {
        println!("Simulation took {:?}", duration);
    }
    Ok(())
}
