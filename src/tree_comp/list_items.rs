use std::{fs::read_dir, io::ErrorKind, path::PathBuf};

use super::error::{TreeCompError, TreeCompErrorKind};

/// Return values like logical OR.
///
/// ```txt
/// paths[0]: a b   d
/// paths[1]: a   c d
/// -----------------
/// return    a b c d
/// ```
pub fn list_items(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut entry = Vec::<PathBuf>::new();

    for parent_path in paths {
        match read_dir(parent_path) {
            Ok(read_dir) => {
                for result in read_dir {
                    match result {
                        Ok(child) => {
                            let path = child.path();
                            let path = match path.strip_prefix(parent_path) {
                                Ok(item) => item,
                                Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                            };
                            if !entry.contains(&path.to_path_buf()) {
                                entry.push(path.to_path_buf())
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
    return entry;
}

/// Search recursively and return possible directories and files.
///
/// ```txt
/// origin[0]: a b   d
/// origin[1]: a   c d
/// ------------------
/// return     a b c d
/// ```
pub fn list_all_relative_paths(origins: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut all_relative_paths = Vec::<PathBuf>::new();
    let mut stack = list_items(&origins);

    while 0 < stack.len() {
        match stack.pop() {
            Some(unexplored) => {
                let mut next_targets = Vec::<PathBuf>::new();
                for path_from_origin in origins {
                    let item = path_from_origin.join(unexplored.to_path_buf());
                    if item.is_dir() {
                        next_targets.push(item)
                    }
                }
                let next_targets = list_items(&next_targets);
                for t in next_targets {
                    stack.push(unexplored.join(t));
                }
                all_relative_paths.push(unexplored);
            }
            None => {}
        }
    }
    return all_relative_paths;
}
