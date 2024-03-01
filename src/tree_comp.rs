use self::{
    list_items::{get_xxhash, list_all_relative_paths},
    path_identity::PathIdentity,
};
use std::path::{PathBuf, MAIN_SEPARATOR};

pub mod error;
mod list_items;
mod path_identity;

pub fn tree_comp(origins: &Vec<PathBuf>, verbose: bool) -> Vec<PathIdentity> {
    let origins = origins.clone();
    let mut nf_paths = Vec::<PathIdentity>::new();

    // list up all relative paths.
    let all_relative_paths = list_all_relative_paths(&origins, verbose);

    // iteration origins and relative paths.
    for origin in origins {
        for relative_path in &all_relative_paths {
            let unknown_item = origin.join(relative_path.path.to_path_buf());
            if unknown_item.exists() {
                if unknown_item.is_dir() {
                    continue;
                }
                if verbose {
                    let target_hash = get_xxhash(&unknown_item, !verbose);
                    let target_id = PathIdentity::new(&unknown_item, target_hash);
                    for relative_path in &all_relative_paths {
                        let full_path = origin.join(relative_path.path.to_path_buf());
                        if full_path == unknown_item {
                            if relative_path.hash != target_hash {
                                nf_paths.push(target_id);
                                break;
                            }
                        }
                    }
                }
            } else {
                if verbose {
                    nf_paths.push(PathIdentity::new(&unknown_item, relative_path.hash))
                } else {
                    let last_item = match nf_paths.last() {
                        Some(item) => match item.path.to_str() {
                            Some(s) => {
                                let mut s = String::from(s);
                                s.push(MAIN_SEPARATOR);
                                s
                            }
                            None => {
                                nf_paths.push(PathIdentity::new(&unknown_item, relative_path.hash));
                                continue;
                            }
                        },
                        None => {
                            nf_paths.push(PathIdentity::new(&unknown_item, relative_path.hash));
                            continue;
                        }
                    };
                    let target_item = match unknown_item.to_str() {
                        Some(s) => s,
                        None => {
                            nf_paths.push(PathIdentity::new(&unknown_item, relative_path.hash));
                            continue;
                        }
                    };
                    if !target_item.contains(&last_item) {
                        nf_paths.push(PathIdentity::new(&unknown_item, relative_path.hash));
                    }
                }
            }
        }
    }
    nf_paths
}
