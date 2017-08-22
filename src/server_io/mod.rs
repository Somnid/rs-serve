use std::path::Path;
use std::fs::File;
use std::io::{Result, Read};
use serde_json;
use serde::de::DeserializeOwned;

pub fn read_file_bytes<T: AsRef<Path>>(path: T) -> Result<Vec<u8>>  {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn read_file_string(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_json<'de, T>(path: String) -> Result<T>
    where T: DeserializeOwned {
    let mut file = File::open(path)?;
    let data = serde_json::from_reader(file)?;
    Ok(data)
}
