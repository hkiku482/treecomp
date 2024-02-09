use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

pub struct OsArgs {
    file_paths: Vec<PathBuf>,
    is_verbose: bool,
}

impl OsArgs {
    pub fn parse() -> Self {
        let arg_id = "directory";
        let option_verbose = "option_verbose";
        let cmd = Command::new("treecomp")
            .arg(
                Arg::new(arg_id)
                    .help("Directory trees to compare")
                    .num_args(2..),
            )
            .arg(
                Arg::new(option_verbose)
                    .help("Use verbose output")
                    .short('v')
                    .long("verbose")
                    .action(ArgAction::SetTrue)
                    .required(false),
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
            is_verbose: cmd.get_flag(option_verbose),
        }
    }

    pub fn get_paths(&self) -> Vec<PathBuf> {
        self.file_paths.clone()
    }

    pub fn is_verbose(&self) -> bool {
        self.is_verbose
    }
}
