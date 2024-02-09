use std::path::{PathBuf, MAIN_SEPARATOR};

use self::list_items::list_all_relative_paths;

pub mod error;
mod list_items;

pub fn tree_comp(origins: &Vec<PathBuf>, verbose: bool) -> Vec<PathBuf> {
    let origins = origins.clone();
    let mut not_found_absolute_paths = Vec::<PathBuf>::new();

    // list up all relative paths.
    let all_relative_paths = list_all_relative_paths(&origins);

    // iteration origins and relative paths.
    for origin in origins {
        for r_path in &all_relative_paths {
            let target = origin.join(r_path);
            if !target.exists() {
                if verbose {
                    not_found_absolute_paths.push(target)
                } else {
                    let last_item = match not_found_absolute_paths.last() {
                        Some(item) => match item.to_str() {
                            Some(s) => {
                                let mut s = String::from(s);
                                s.push(MAIN_SEPARATOR);
                                s
                            }
                            None => {
                                not_found_absolute_paths.push(target);
                                continue;
                            }
                        },
                        None => {
                            not_found_absolute_paths.push(target);
                            continue;
                        }
                    };
                    let target_item = match target.to_str() {
                        Some(s) => s,
                        None => {
                            not_found_absolute_paths.push(target);
                            continue;
                        }
                    };
                    if !target_item.contains(&last_item) {
                        not_found_absolute_paths.push(target);
                    }
                }
            }
        }
    }

    return not_found_absolute_paths;
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    #[test]
    fn t() {
        let p = PathBuf::from("A");
        let p = p.join("a");
        println!("{:?}", p)
    }
}
