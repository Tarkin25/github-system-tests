use configuration::{init_logging, Configuration};

mod configuration;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Configuration::new()?;
    init_logging(&config);

    tracing::debug!("{config:?}");

    Ok(())
}
