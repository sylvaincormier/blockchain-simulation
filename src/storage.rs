use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub accounts: HashMap<String, u64>,
}

impl Storage {
    pub fn load() -> Result<Self, String> {
        let file = File::open("blockchain.db").map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| e.to_string())
    }

    pub fn save(&self) -> Result<(), String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("blockchain.db")
            .map_err(|e| e.to_string())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self).map_err(|e| e.to_string())
    }
}
