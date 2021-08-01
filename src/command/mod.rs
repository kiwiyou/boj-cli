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

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub language: String,
    pub created_at: NaiveDate,
    pub solved: bool,
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

pub fn load_meta(prob_dir: impl AsRef<Path>) -> eyre::Result<Metadata> {
    let path = prob_dir.as_ref().join("meta.json");
    if !path.exists() {
        bail!("meta.json doesn't exist");
    }
    let meta = serde_json::from_reader(std::io::BufReader::new(File::open(path)?))?;
    Ok(meta)
}

pub fn save_meta(prob_dir: impl AsRef<Path>, meta: &Metadata) -> eyre::Result<()> {
    let path = prob_dir.as_ref().join("meta.json");
    serde_json::to_writer_pretty(std::io::BufWriter::new(File::create(path)?), meta)?;
    Ok(())
}
