use std::path::PathBuf;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct PathIdentity {
    pub path: PathBuf,
    pub hash: Option<u64>, // None means directory.
}

impl PathIdentity {
    pub fn new(path: &PathBuf, hash: Option<u64>) -> Self {
        Self {
            path: path.clone(),
            hash,
        }
    }
}
