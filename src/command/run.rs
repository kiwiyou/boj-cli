use std::fs::copy;
use std::process::Command;

use clap::{AppSettings, Clap};
use eyre::bail;
use log::info;
use tempfile::tempdir;

use crate::command::get_solution_file;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Run {
    /// Problem ID
    id: u32,
}

pub fn run(arg: Run) -> eyre::Result<()> {
    let (path, lang) = get_solution_file(arg.id)?;
    let temp_dir = tempdir()?;
    let main_path = temp_dir.path().join("Main").with_extension(lang.ext);
    copy(&path, &main_path)?;
    if let Some(compile) = lang.compile {
        info!("compiling the solution");
        let exit = Command::new("bash")
            .args(&["-c", compile])
            .current_dir(temp_dir.path())
            .spawn()?
            .wait()?;
        if !exit.success() {
            bail!("compilation failed");
        }
    }
    info!("running the solution");
    let exit = Command::new("bash")
        .args(&["-c", lang.execute])
        .current_dir(temp_dir.path())
        .spawn()?
        .wait()?;
    if !exit.success() {
        bail!("NZEC");
    }
    Ok(())
}
