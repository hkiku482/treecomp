use treecomp::tree_comp::tree_comp;

use crate::cmd_args::OsArgs;
mod cmd_args;

fn main() {
    let args = OsArgs::parse();
    let origins = args.get_paths();
    let paths = tree_comp(&origins, args.is_verbose());
    print!("\x1b[31m");
    for path in paths {
        let item_name = match path.to_str() {
            None => "None",
            Some(s) => s,
        };
        println!("- {}", item_name)
    }
    print!("\x1b[0m");
}
