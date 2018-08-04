extern crate serde_json;

use serde::de::DeserializeOwned;
use std::path::Path;
use std::fs::File;

pub fn load_json_from_file<T: DeserializeOwned>(path: &Path) -> T {
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).unwrap()
}
