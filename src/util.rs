extern crate serde_json;

use serde::de::DeserializeOwned;
use std::fs::File;
use std::io;
use std::path::Path;

pub enum JSONIOError {
    JSONError(serde_json::Error),
    IOError(io::Error),
}

pub fn load_json_from_file<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T, JSONIOError> {
    let file = File::open(path).map_err(|e| JSONIOError::IOError(e))?;
    serde_json::from_reader(file).map_err(|e| JSONIOError::JSONError(e))
}
