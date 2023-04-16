use std::{fs::read_dir, path::PathBuf};

use crate::error::{TreeCompError, TreeCompErrorKind};

#[derive(Debug)]
pub struct Missing {
    parent: PathBuf,
    entry: PathBuf,
}

impl Missing {
    pub fn new(origin: PathBuf, relative_path: PathBuf) -> Self {
        Self {
            parent: origin,
            entry: relative_path,
        }
    }

    pub fn get_origin(&self) -> PathBuf {
        self.parent.clone()
    }

    pub fn get_full_path(&self) -> PathBuf {
        self.parent.join(&self.entry)
    }
}

pub fn count_missing(missing_items: &Vec<Missing>, entry: &PathBuf) -> usize {
    let mut count: usize = 0;
    for item in missing_items {
        if &item.entry == entry {
            count += 1;
        }
    }
    count
}

pub fn list_all_entries(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut all_entries = Vec::<PathBuf>::new();

    for parent in paths {
        match read_dir(parent) {
            Ok(children) => {
                for res in children {
                    match res {
                        Ok(child) => {
                            let p = child.path();
                            let p = match p.strip_prefix(parent) {
                                Ok(v) => v,
                                Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                            };
                            if !all_entries.contains(&p.to_path_buf()) {
                                all_entries.push(p.to_path_buf());
                            }
                        }
                        Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                    }
                }
            }
            Err(_) => {
                if parent.exists() {
                    TreeCompError::new(TreeCompErrorKind::FatalError).exit()
                } else {
                    TreeCompError::new(TreeCompErrorKind::DirectoryNotFound(parent.clone())).exit();
                }
            }
        }
    }
    all_entries
}

pub fn has_entry(parent: &PathBuf, entry: &PathBuf) -> bool {
    let target = parent.join(entry);
    match read_dir(parent) {
        Ok(children) => {
            for res in children {
                match res {
                    Ok(child) => {
                        if &child.path() == &target {
                            return true;
                        }
                    }
                    Err(_) => TreeCompError::new(TreeCompErrorKind::FatalError).exit(),
                }
            }
        }
        Err(_) => {
            if parent.exists() {
                TreeCompError::new(TreeCompErrorKind::FatalError).exit()
            } else {
                TreeCompError::new(TreeCompErrorKind::DirectoryNotFound(parent.clone())).exit();
            }
        }
    }
    false
}
