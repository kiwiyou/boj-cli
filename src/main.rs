use std::fs::{DirBuilder, File};
use std::path::Path;

use chrono::NaiveDate;
use clap::{crate_authors, crate_version, AppSettings, Clap};
use log::{error, info};
use tera::Tera;
use vendor::Client;

mod vendor;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about("Create and initialize a new problem directory"))]
    New(New),
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct New {
    #[clap(about("Problem ID"))]
    id: u32,
    #[clap(short, long, about("Title of the problem in kebab-case"))]
    title: Option<String>,
}

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    pretty_env_logger::formatted_builder()
        .filter(Some("boj_cli"), log::LevelFilter::Info)
        .init();

    match opts.subcmd {
        SubCommand::New(arg) => new(arg)?,
    }
    Ok(())
}

fn new(arg: New) -> eyre::Result<()> {
    let client = Client::new();
    let problem = client.find_problem_by_id(arg.id)?;
    let problem = if let Some(problem) = problem {
        problem
    } else {
        error!("problem with id {} doesn't exist", arg.id);
        return Ok(());
    };
    let dir_name = if let Some(title) = arg.title {
        format!("{}-{}", arg.id, title)
    } else {
        arg.id.to_string()
    };
    let path = Path::new(&dir_name);
    if path.exists() {
        error!(
            "destination `{}` already exists\n\
        \n\
        Use `boj-cli init` to initialize the directory",
            path.display()
        );
        return Ok(());
    }
    DirBuilder::new().create(path)?;
    info!("created problem directory");

    let mut meta = File::create(path.join("meta.json"))?;
    serde_json::to_writer_pretty(
        &mut meta,
        &Metadata {
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

    let mut src = File::create(path.join("solution.rs"))?;
    tera.render_to("solution.rs", &context, &mut src)?;
    info!("generated source file");

    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: NaiveDate,
    pub solved: bool,
}
