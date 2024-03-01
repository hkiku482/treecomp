use super::{
    error::{TreeCompError, TreeCompErrorKind},
    path_identity::PathIdentity,
};
use std::{
    fs::{read, read_dir},
    io::ErrorKind,
    path::PathBuf,
};
use xxhash_rust::xxh3::xxh3_64;

pub fn get_xxhash(p: &PathBuf, no_read: bool) -> Option<u64> {
    if no_read {
        return None;
    }
    if p.is_file() {
        let hash;
        match read(p) {
            Ok(f) => {
                hash = xxh3_64(&f.as_slice());
            }
            Err(_) => hash = 0,
        };
        Some(hash)
    } else {
        None
    }
}

/// Return values like logical OR.
///
/// ```txt
/// paths[0]: a b   d
/// paths[1]: a   c d
/// -----------------
/// return    a b c d
/// ```
pub fn list_items(paths: &Vec<PathBuf>, verbose: bool) -> Vec<PathIdentity> {
    let mut entry = Vec::<PathIdentity>::new();

    for parent_path in paths {
        match read_dir(parent_path) {
            Ok(read_dir) => {
                for item in read_dir {
                    match item {
                        Ok(child) => {
                            let path = child.path();
                            let path = match path.strip_prefix(parent_path) {
                                Ok(p) => p,
                                Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                            };
                            let file_identiry;
                            let p = parent_path.clone().join(path);
                            let hash = get_xxhash(&p, !verbose);
                            file_identiry = PathIdentity::new(&path.to_path_buf(), hash);

                            if !entry.contains(&file_identiry) {
                                entry.push(file_identiry);
                            }
                        }
                        Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                    }
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    TreeCompError::new(TreeCompErrorKind::PermissionDenied(
                        parent_path.to_path_buf(),
                    ))
                    .show_log();
                }
            }
        }
    }
    entry
}

/// Search recursively and return possible directories and files.
///
/// ```txt
/// origin[0]: a b   d
/// origin[1]: a   c d
/// ------------------
/// return     a b c d
/// ```
pub fn list_all_relative_paths(origins: &Vec<PathBuf>, verbose: bool) -> Vec<PathIdentity> {
    let mut all_relative_paths = Vec::<PathIdentity>::new();
    let mut stack = list_items(&origins, verbose);

    while 0 < stack.len() {
        match stack.pop() {
            Some(unexplored) => {
                let unexplored_path = &unexplored.path;
                let mut next_targets = Vec::<PathBuf>::new();
                for path_from_origin in origins {
                    let item = path_from_origin.join(unexplored_path);
                    if item.is_dir() {
                        next_targets.push(item)
                    }
                }
                let next_targets = list_items(&next_targets, verbose);
                for t in next_targets {
                    stack.push(PathIdentity::new(&unexplored_path.join(t.path), t.hash));
                }
                all_relative_paths.push(unexplored);
            }
            None => {}
        }
    }
    all_relative_paths
}
