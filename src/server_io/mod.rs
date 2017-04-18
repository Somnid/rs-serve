use std::path::Path;
use std::fs::File;
use std::io::Read;
use serde_json;
use serde::Deserialize;

pub fn read_file_bytes<T: AsRef<Path>>(path: T) -> Result<Vec<u8>, String>  {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file|{
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_|contents)
        })
}

fn read_file_string(path: &str) -> Result<String, String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file|{
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_|contents)
        })
}

pub fn read_file_json<T>(path: String) -> Result<T, String>
    where T: Deserialize {
    read_file_string(&path)
        .and_then(|text| {
            serde_json::from_str::<T>(text.as_str())
                .map_err(|e| e.to_string())
        })
}
