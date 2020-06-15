use clap::Clap;

use crate::lib::observe::git::detect::detect;

/// Detect changes and trigger build instructions
#[derive(Clap)]
pub struct Build {
    /// The directory to scan for changes to build
    #[clap(short = "d", long = "directory", default_value = ".")]
    directory: String,
}

// Option<Result<Oid, Error>>
pub fn build(args: Build) {
    let paths = match detect(args.directory.as_str()) {
        Ok(f) => f,
        Err(_e) => return,
    };

    for path in paths {
        println!("We need to build {}", path.to_str().unwrap())
    }

    //

    return;
}
