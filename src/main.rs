use clap::Parser;
use grepe::{Config, run};

fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    run(config)
}
