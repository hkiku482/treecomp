use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
    process,
};

#[derive(Debug)]
pub struct TreeCompError {
    kind: TreeCompErrorKind,
}

#[derive(Debug)]
pub enum TreeCompErrorKind {
    FatalError,
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

    /// Print stderr and exit.
    pub fn exit(&self) -> ! {
        match io::stderr().write_all(self.msg().as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!(),
        };
        process::exit(1)
    }

    fn msg(&self) -> &str {
        match self.kind {
            TreeCompErrorKind::FatalError => "fatal error has occurred",
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
        self.msg()
    }
}
