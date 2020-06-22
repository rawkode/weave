use clap::Clap;
use std::path::PathBuf;

use crate::lib::detect::detect_build_roots;
use crate::lib::observe::git::detect;

/// Detect changes and trigger build instructions
#[derive(Clap)]
pub struct Build {
    /// The directory to scan for changes to build
    #[clap(short = "d", long = "directory", default_value = ".")]
    directory: String,
}

// Option<Result<Oid, Error>>
pub fn build(args: Build) {
    let directory = PathBuf::from(args.directory.as_str());

    let paths = match detect(&directory) {
        Ok(f) => f,
        Err(_e) => return,
    };

    //
    let build_roots = detect_build_roots(&paths);

    for dir in build_roots {
        log::info!("Building {}", dir.to_str().unwrap());
    }
}
