use color_eyre::Result;
use std::env::{set_var, var};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, EnvFilter};

fn main() -> Result<()> {
    color_eyre::install()?;

    if var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "INFO");
    }

    registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();

    Ok(())
}
