use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::{container::StopContainerOptions, Docker};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use std::process;

// Executes the `zweb stop` command to
// stop one or more ZWEB Builder
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("stop")
        .required(true)
        .args(&["self_host", "cloud"]),
))]
/// Stop one or more ZWEB Builder
pub struct Cmd {
    /// Stop Self-hosted ZWEB Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// Stop ZWEB Builder on ZWEB Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => stop_local().await?,
            (_, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn stop_local() -> Result {
    println!("{} Trying to stop the ZWEB Builder...", ui::emoji::BUILD);

    let _docker = Docker::connect_with_local_defaults().unwrap();
    if (_docker.ping().await).is_err() {
        println!(
            "{} {}\n{} {}",
            ui::emoji::FAIL,
            String::from("No running docker found."),
            ui::emoji::WARN,
            style("Please check the status of docker.").red(),
        );
        process::exit(1);
    }

    let options = Some(StopContainerOptions { t: 30 });
    let stop_builder = _docker.stop_container("zweb_builder", options).await;
    if stop_builder.is_err() {
        println!(
            "{} {} {}",
            ui::emoji::FAIL,
            String::from("Try to stop ZWEB Builder error:"),
            style(stop_builder.err().unwrap()).red(),
        );
        process::exit(1);
    }

    println!(
        "{} {}",
        ui::emoji::SUCCESS,
        style("Successfully stop the ZWEB Builder.").green(),
    );

    Ok(())
}
