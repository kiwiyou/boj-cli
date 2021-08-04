use std::fs::File;
use std::path::Path;

use chrono::NaiveDate;
use clap::Clap;
use eyre::bail;
use log::error;
use serde::{Deserialize, Serialize};
mod new;
mod run;

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about("Create and initialize a new problem directory"))]
    New(new::New),
    #[clap(about("Compile and run the solution"))]
    Run(run::Run),
}

pub fn dispatch(cmd: SubCommand) {
    let result = match cmd {
        SubCommand::New(arg) => new::new(arg),
        SubCommand::Run(arg) => run::run(arg),
    };

    if let Err(e) = result {
        error!("{}", e);
    }
}
