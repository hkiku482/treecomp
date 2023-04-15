use std::{fs::read_dir, path::PathBuf};

use crate::tree_comp::{list_all_entries, Missing};

use super::error::{TreeCompError, TreeCompErrorKind};

/// Compare directory trees.
///
/// # Example
/// Directories(`path: &Vec<PathBuf>`) have
/// ```txt
/// dir1  : a b   d
/// dir2  : a   c d
/// ---------------
/// return:   b c
/// ```
pub fn directory(paths: &Vec<PathBuf>) -> Vec<Missing> {
    let mut diff = Vec::<Missing>::new();
    for parent_path in paths {
        for entry in &list_all_entries(&paths) {
            match read_dir(parent_path) {
                Ok(children) => {
                    let mut found = false;
                    for res in children {
                        match res {
                            Ok(child) => {
                                let e = parent_path.join(entry);
                                if child.path() == e {
                                    found = true;
                                    break;
                                }
                            }
                            Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                        }
                    }
                    if !found {
                        diff.push(Missing::new(
                            parent_path.clone(),
                            match entry.to_path_buf().file_name() {
                                Some(p) => PathBuf::from(p),
                                None => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                            },
                        ));
                    }
                }
                Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
            }
        }
    }
    diff
}
