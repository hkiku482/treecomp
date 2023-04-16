use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
    path::PathBuf,
    process,
};

#[derive(Debug)]
pub struct TreeCompError {
    kind: TreeCompErrorKind,
}

#[derive(Debug)]
pub enum TreeCompErrorKind {
    FatalError,
    DirectoryNotFound(PathBuf),
}

impl Default for TreeCompErrorKind {
    fn default() -> Self {
        Self::FatalError
    }
}

impl TreeCompError {
    pub fn new(kind: TreeCompErrorKind) -> Self {
        Self { kind }
    }

    /// Print stderr.
    pub fn show_log(&self) {
        match io::stderr().write_all(format!("\x1b[33m{}\x1b[0m\n", self.msg()).as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!(),
        };
    }

    /// Print stderr and exit.
    pub fn exit(&self) -> ! {
        self.show_log();
        process::exit(1)
    }

    fn msg(&self) -> String {
        match &self.kind {
            TreeCompErrorKind::FatalError => "\x1b[31mfatal error has occurred\x1b[0m".to_string(),
            TreeCompErrorKind::DirectoryNotFound(p) => {
                let p = match p.to_str() {
                    Some(s) => s,
                    None => "directory",
                };
                format!("\"{p}\" not found")
            }
        }
    }
}

impl Display for TreeCompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg())
    }
}

impl Error for TreeCompError {
    fn description(&self) -> &str {
        "treecomp error"
    }
}
