use treecomp::tree_comp::tree_comp;

use crate::cmd_args::OsArgs;
mod cmd_args;

fn main() {
    let args = OsArgs::parse();
    let origins = args.get_paths();
    let path_identities = tree_comp(&origins, args.is_verbose());
    print!("\x1b[31m");
    for path_identity in path_identities {
        let path = match path_identity.path.to_str() {
            None => "None",
            Some(s) => s,
        };
        let hash = match path_identity.hash {
            Some(h) => format!("{:x}", h),
            None => "".to_string(),
        };
        if args.is_verbose() {
            println!("- {}: {}", hash, path)
        } else {
            println!("- {}", path)
        }
    }
    print!("\x1b[0m");
}
