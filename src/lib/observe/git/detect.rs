use git2::Error;
use std::{collections::HashSet, path::PathBuf};

// I hate that this line of code needs to exist
// Rust doesn't like modules be broken down to a
// function level.
use super::verify::verify;

///
/// Detects changes in the Git repository by walking back
/// to the previous commit.
///
pub fn detect(directory: &str) -> Result<HashSet<PathBuf>, Error> {
    let repo = verify(directory)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let rev = revwalk.find(|walk| {
        let rev = match walk {
            &Ok(r) => r,
            &Err(_) => return true,
        };
        match repo.find_commit(rev) {
            Ok(_commit) => return true,
            Err(_) => return false,
        };
    });

    let commit = match repo.find_commit(rev.unwrap()?) {
        Ok(commit) => commit,
        Err(e) => return Err(e),
    };

    let parent_commit = commit.parent(0)?;

    let diff = repo
        .diff_tree_to_tree(
            Some(&commit.tree().unwrap()),
            Some(&parent_commit.tree().unwrap()),
            None,
        )
        .unwrap();
    let ds = diff.deltas();

    let mut dirs = HashSet::new();

    for d in ds {
        let dir = d.new_file().path().unwrap().parent().unwrap().to_owned();
        log::info!("Modified dir is {}", dir.to_str().unwrap());
        dirs.insert(dir);
    }

    return Ok(dirs);
}
