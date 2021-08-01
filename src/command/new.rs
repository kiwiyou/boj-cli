use std::fs::{DirBuilder, File};
use std::path::Path;

use clap::{AppSettings, Clap};
use eyre::{bail, eyre};
use log::info;
use tera::Tera;

use crate::command::{save_meta, Metadata};
use crate::language;
use crate::vendor::Client;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct New {
    /// Problem ID
    id: u32,
    /// Language to use
    lang: String,
    /// Problem Title in kebab-case
    #[clap(short, long)]
    title: Option<String>,
}

pub fn new(arg: New) -> eyre::Result<()> {
    let client = Client::new();
    let lang = language::find_by_short(&arg.lang)
        .ok_or_else(|| eyre!("language `{}` doesn't exist", &arg.lang))?;
    let problem = client
        .find_problem_by_id(arg.id)?
        .ok_or_else(|| eyre!("problem with id {} doesn't exist", arg.id))?;
    let dir_name = if let Some(title) = arg.title {
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

    save_meta(
        path,
        &Metadata {
            language: lang.short.to_string(),
            created_at: chrono::Local::today().naive_local(),
            solved: false,
        },
    )?;

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
