use crate::cli::args::get_password_store_path;
use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{read_to_string, File};
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::string::ToString;

// TODO: add incorrect_password_entered type
pub enum LogType {
    UpdateMasterPassword,
    GeneratePassword,
    AddPassword,
    RemovePassword,
    List,
    Show,
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    event: String,
    time_stamp: String,
    extra: Option<String>,
}

impl Log {
    fn new(log_type: LogType, extra: Option<String>) -> Self {
        let event = match log_type {
            LogType::AddPassword => "add_password".to_string(),
            LogType::List => "list".to_string(),
            LogType::RemovePassword => "remove_password".to_string(),
            LogType::GeneratePassword => "generate_password".to_string(),
            LogType::Show => "show".to_string(),
            LogType::UpdateMasterPassword => "update_master".to_string(),
        };
        let time_stamp = Local::now().to_string();
        Self {
            event,
            time_stamp,
            extra,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LogsData {
    logs: Vec<Log>,
}

pub struct Logger {
    path: PathBuf,
}

impl Logger {
    pub fn new() -> Self {
        let file_name = "logs.json".to_string();
        let path = get_password_store_path(file_name).unwrap();
        let log_file = File::open(&path);
        match log_file {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    let john = json!({
                        "logs": []
                    });
                    let mut file = File::create(&path).unwrap();
                    file.write_all(john.to_string().as_bytes()).unwrap();
                }
                _ => (),
            },
            _ => (),
        }
        Self { path }
    }

    // Write log to the file
    pub fn write_log(&self, log_type: LogType, extra: Option<String>) {
        let file_stringified = read_to_string(&self.path).unwrap();
        let mut json_data: LogsData = serde_json::from_str(&file_stringified).unwrap();
        let log = Log::new(log_type, extra);
        json_data.logs.push(log);
        let json_value = serde_json::to_string_pretty(&json_data).unwrap();
        std::fs::write(&self.path, json_value).unwrap();
    }
}
