use clap::{crate_authors, crate_version, AppSettings, Clap};

mod command;
mod language;
mod vendor;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: command::SubCommand,
}

fn main() {
    let opts = Opts::parse();
    pretty_env_logger::formatted_builder()
        .filter(Some("boj"), log::LevelFilter::Info)
        .init();
    command::dispatch(opts.subcmd)
}
