use std::fs::read_dir;
use std::path::PathBuf;

use clap::Clap;
use eyre::bail;
use log::error;
use muroba::query::{Query, QueryBuilder};

use crate::language::{self, Language};
mod new;
mod run;

#[derive(Clap)]
pub enum SubCommand {
    /// Create and initialize a new problem directory
    New(new::New),
    /// Compile and run the solution
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

fn get_solution_file(id: u32) -> eyre::Result<(PathBuf, &'static Language)> {
    if let Some(dir) = find_prob_dir(id)? {
        let mut available = vec![];
        for entry in read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if matches!(path.file_stem(), Some(stem) if stem == "solution") {
                if let Some(ext) = path.extension() {
                    language::find_by_ext(ext)
                        .for_each(|lang| available.push((path.clone(), lang)));
                }
            }
        }
        if available.is_empty() {
            bail!("No solution file exists");
        } else if available.len() == 1 {
            Ok(available.remove(0))
        } else {
            let choices: Vec<_> = available.iter().map(|(_, l)| l.name).collect();
            let select = QueryBuilder::default()
                .with_prompt("Choose the language to run with")
                .select(&choices)
                .fix_rows(5)
                .show()?;
            Ok(available.swap_remove(select[0].0))
        }
    } else {
        bail!(
            "The problem directory doesn't exist\n\
            Expected `{0}` or `{0}-...`",
            id
        );
    }
}
