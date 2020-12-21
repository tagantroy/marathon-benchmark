mod adb_server;
mod benchmark_results;
mod config;
mod devices;
mod file_manager;
mod run_config;
mod test_suite;
mod testrunners;
mod tools;

use console::{style, Emoji};

use crate::run_config::RunConfig;
use clap::{App, Arg};
use config::Config;
use std::time::Instant;
use test_suite::TestSuiteRunner;
use uuid::Uuid;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let run_uuid = Uuid::new_v4();
    let start_time = Instant::now();
    let current_dir = std::env::current_dir()?;
    let run_config = RunConfig::new(run_uuid, current_dir, start_time);
    let matches = App::new("marathonbm")
        .version("0.1")
        .author("Ivan Balaksha <tagantroy@gmail.com>")
        .about("Benchmark tool for marathon test runner")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Specify config name")
                .takes_value(true),
        )
        .get_matches();

    println!("RUN ID: {}", &run_config.uuid);
    println!(
        "{} {}Processing configuration...",
        style("[1/3]").bold().dim(),
        LOOKING_GLASS
    );

    let config_path = matches.value_of("config").unwrap_or("default.yaml");
    let config_content = std::fs::read_to_string(config_path)?;

    let config: Config = serde_yaml::from_str(&config_content)?;

    println!(
        "{} {}Running test suites...",
        style("[2/3]").bold().dim(),
        TRUCK
    );

    let mut suites: Vec<TestSuiteRunner> = config.into();

    for suite in suites.iter_mut() {
        suite.start(&run_config).await?;
    }

    println!("{} {}Saving results...", style("[3/3]").bold().dim(), PAPER);
    Ok(())
}
