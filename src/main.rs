use std::path::PathBuf;

use crate::cmd_args::OsArgs;
use treecomp::{
    compare,
    tree_comp::{has_entry, list_all_entries, Missing},
};
mod cmd_args;
fn main() {
    let origin = OsArgs::parse().get_paths();

    let mut parents = origin.clone();
    let mut unexplore_entries = Vec::<PathBuf>::new();

    loop {
        let missing_entries = compare::directory(&parents);
        print_missing(&missing_entries);

        let current_entries = list_all_entries(&parents);

        unexplore_entries.append(&mut list_directories(&parents, &current_entries));
        parents = list_parents(
            &parents,
            match &unexplore_entries.pop() {
                Some(entry) => entry,
                None => break,
            },
        );
    }
}

fn print_missing(items: &Vec<Missing>) {
    let color_red = "\x1b[31m";
    let color_def = "\x1b[0m";

    print!("{}", color_red);
    for item in items {
        let path = item.get_full_path();
        let path = path.to_str();
        let path = match path {
            Some(s) => String::from(s),
            None => String::from("None"),
        };
        println!("- {}", path);
    }
    print!("{}", color_def);
}

fn list_directories(parents: &Vec<PathBuf>, entries: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unexplore = Vec::<PathBuf>::new();
    for entry in entries {
        for parent in parents {
            if has_entry(parent, entry) {
                if parent.join(entry).is_dir() {
                    unexplore.push(entry.clone());
                    break;
                }
            }
        }
    }
    unexplore
}

fn list_parents(parents: &Vec<PathBuf>, entry: &PathBuf) -> Vec<PathBuf> {
    let mut entries = Vec::<PathBuf>::new();
    for parent in parents {
        if has_entry(parent, entry) {
            let next = parent.join(entry);
            if next.is_dir() {
                entries.push(next);
            }
        }
    }
    entries
}
