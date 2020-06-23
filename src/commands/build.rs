use clap::Clap;
use std::path::PathBuf;

use crate::lib::detect::detect_build_roots;

use crate::lib::build_tools::{BuildTool, BuildTools};

use crate::lib::observe::all::AllConfig;
use crate::lib::observe::git::detect;
use crate::lib::observe::Observer;

/// Detect changes and trigger build instructions
#[derive(Clap)]
pub struct Build {
    /// The directory to scan for changes to build
    #[clap(short = "d", long = "directory", default_value = ".")]
    directory: String,

    /// Modes
    #[clap(short = "m", long = "mode", default_value = "ci", possible_values=&["all", "ci"])]
    mode: String,
}

pub fn build(args: Build) {
    let directory = PathBuf::from(args.directory.as_str());
    let mode = args.mode.as_str();

    let paths = match mode {
        "all" => {
            let all_config = AllConfig {
                directory: directory.clone(),
            };

            all_config.observe()
        }
        "ci" => match detect(&directory) {
            Ok(f) => f,
            Err(_e) => return,
        },
        _ => return,
    };

    //
    let build_roots = detect_build_roots(&directory, &paths);

    for build_root in build_roots {
        println!("\n");

        match build_root {
            BuildTools::Docker(docker) => {
                println!(
                    "Found a Docker build, with Dockerfile {}, inside {}\n",
                    docker.dockerfile,
                    docker.config.directory.to_str().unwrap(),
                );

                match docker.build() {
                    true => println!("Build Successfully"),
                    false => println!("Build Failed"),
                }
            }
            BuildTools::GitLab(gitlab) => {
                println!(
                    "Found a GitLab build inside {}\n",
                    gitlab.config.directory.to_str().unwrap()
                );
                match gitlab.build() {
                    true => println!("Build Successfully"),
                    false => println!("Build Failed"),
                }
            }
        }
    }
}
