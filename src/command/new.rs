use std::fs::{DirBuilder, File};
use std::path::Path;

use clap::{AppSettings, Clap};
use eyre::{bail, eyre};
use log::info;
use muroba::query::{Query, QueryBuilder};
use tera::Tera;

use crate::language::LANGUAGES;
use crate::vendor::Client;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct New {
    /// Problem ID
    id: u32,
}

pub fn new(arg: New) -> eyre::Result<()> {
    let client = Client::new();
    let problem = client
        .find_problem_by_id(arg.id)?
        .ok_or_else(|| eyre!("problem with id {} doesn't exist", arg.id))?;

    let language_list: Vec<_> = LANGUAGES.iter().map(|l| l.name).collect();
    let select = QueryBuilder::default()
        .with_prompt("Choose the langauge")
        .select(&language_list)
        .fix_rows(5)
        .show()?;
    let lang = &LANGUAGES[select[0].0];

    let title = QueryBuilder::default()
        .with_prompt("Optional title")
        .input()
        .show()?;

    let dir_name = if !title.is_empty() {
        format!("{}-{}", arg.id, title)
    } else {
        arg.id.to_string()
    };

    let path = Path::new(&dir_name);
    if path.exists() {
        bail!(
            "destination `{}` already exists\n\
            \n\
            Use `boj-cli init` to initialize the directory",
            path.display()
        );
    }
    DirBuilder::new().create(path)?;
    info!("created problem directory");

    let tera = Tera::new("template/**/*")?;
    let mut context = tera::Context::new();
    context.insert("problem", &problem);
    let mut readme = File::create(path.join("note.md"))?;
    tera.render_to("note.md", &context, &mut readme)?;
    info!("generated note");

    let solution = path.join("solution").with_extension(lang.ext);
    let mut src = File::create(&solution)?;
    tera.render_to(
        solution.file_name().unwrap().to_str().unwrap(),
        &context,
        &mut src,
    )?;
    info!("generated source file");

    Ok(())
}
