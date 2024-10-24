use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fs::File;
use std::io::{Read, Write};

pub fn write_json<T: Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let json = to_string_pretty(data).expect("Received unserializable data");
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: &str) -> std::io::Result<T> {
    let mut file = File::open(path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    if let Ok(data) = from_str(&json) {
        Ok(data)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid JSON",
        ))
    }
}
