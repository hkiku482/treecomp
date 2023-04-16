use std::path::PathBuf;

use clap::{Arg, Command};

pub struct OsArgs {
    file_paths: Vec<PathBuf>,
}

impl OsArgs {
    pub fn parse() -> Self {
        let arg_id = "directories";
        let cmd = Command::new("treecomp")
            .arg(
                Arg::new(arg_id)
                    .help("directory trees to compare")
                    .num_args(2..),
            )
            .get_matches();

        let mut cmd_arg_values: Vec<PathBuf> = Vec::new();

        match cmd.get_many::<String>(arg_id) {
            Some(args) => {
                for v in args {
                    cmd_arg_values.push(match v.parse::<String>() {
                        Ok(s) => PathBuf::from(s),
                        Err(_) => clap::Error::new(clap::error::ErrorKind::InvalidValue).exit(),
                    })
                }
            }
            None => clap::Error::new(clap::error::ErrorKind::InvalidValue).exit(),
        }

        OsArgs {
            file_paths: cmd_arg_values,
        }
    }

    pub fn get_paths(&self) -> Vec<PathBuf> {
        self.file_paths.clone()
    }
}
