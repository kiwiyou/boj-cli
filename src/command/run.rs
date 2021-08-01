use std::fs::{copy, read_dir};
use std::path::PathBuf;
use std::process::Command;

use clap::{AppSettings, Clap};
use eyre::{bail, eyre};
use log::info;
use tempfile::tempdir;

use crate::language;

use super::load_meta;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Run {
    /// Problem ID
    id: u32,
}

pub fn run(arg: Run) -> eyre::Result<()> {
    if let Some(dir) = find_prob_dir(arg.id)? {
        let meta = load_meta(&dir)?;
        let lang = language::find_by_short(&meta.language).ok_or_else(|| {
            eyre!(
                "error in meta.json; language `{}` doesn't exist",
                &meta.language
            )
        })?;
        let filename = format!("solution.{}", lang.ext);
        for entry in read_dir(&dir)? {
            let entry = entry?;
            if entry.file_name() == filename.as_str() {
                let temp_dir = tempdir()?;
                let main_path = temp_dir.path().join("Main").with_extension(lang.ext);
                copy(entry.path(), &main_path)?;
                if let Some(ref compile) = lang.compile {
                    info!("compiling the solution");
                    let exit = Command::new(&compile.exec)
                        .args(compile.args)
                        .current_dir(temp_dir.path())
                        .spawn()?
                        .wait()?;
                    if !exit.success() {
                        bail!("compilation failed");
                    }
                }
                info!("running the solution");
                let exit = Command::new(lang.execute.exec)
                    .args(lang.execute.args)
                    .current_dir(temp_dir.path())
                    .spawn()?
                    .wait()?;
                if !exit.success() {
                    bail!("NZEC");
                }
                break;
            }
        }
    } else {
        bail!(
            "The problem directory doesn't exist\n\
            Expected `{0}` or `{0}-...`",
            arg.id
        );
    }
    Ok(())
}

fn find_prob_dir(id: u32) -> eyre::Result<Option<PathBuf>> {
    let id_str = id.to_string();
    for entry in read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let stripped = path
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|n| n.strip_prefix(&id_str));
        if matches!(stripped, Some(stripped) if stripped.is_empty() || stripped.starts_with('-')) {
            return Ok(Some(path));
        }
    }
    Ok(None)
}
