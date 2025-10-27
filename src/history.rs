use serde::{Serialize, Deserialize};
use std::fs;
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Default)]
pub struct History {
    pub records: Vec<String>,
}

pub const HISTORY_FILE: &str = "conversion.json";

pub fn load_history() -> Result<History, Box<dyn std::error::Error>> {
    match fs::read_to_string(HISTORY_FILE) {
        Ok(content) => {
            let history: History = serde_json::from_str(&content)?;
            Ok(history)
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            Ok(History::default())
        },
        Err(e) => {
            Err(e.into())
        }
    }
}

pub fn save_to_history(record: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut history = load_history()?;
    history.records.push(record);
    let json_content = serde_json::to_string_pretty(&history)?;
    fs::write(HISTORY_FILE, json_content)?;
    Ok(())
}