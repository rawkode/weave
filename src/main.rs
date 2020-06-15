use clap::Clap;

mod commands;
mod lib;

use commands::build::{build, Build};

/// weave is a build tool that doesn't want to be a build tool.
/// It will detect changes within your Git repository and fork off real
/// build tools when things change.
#[derive(Clap)]
#[clap(version = "0.1.1", author = "David McKay <david@rawkode.com>")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    _verbose: i32,

    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Build(Build),
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn"));

    let opts: Opts = Opts::parse();

    match opts.sub_command {
        SubCommand::Build(args) => build(args),
    }
}
