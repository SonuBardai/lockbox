use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::fs::File;
use std::string::ToString;
use crate::cli::args::get_password_store_path;

// TODO: add incorrect_password_entered type
pub enum LogType {
    AddPassword,
    RemovePassword,
    List,
    Show
}

pub struct Logger{
    path: PathBuf
}

impl Logger {
    pub fn new () -> Self {
        let file_name = "logs.json".to_string();
        let path = get_password_store_path(file_name).unwrap();
        let log_file = File::open(&path);
        match log_file {
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        File::create(&path).unwrap();
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        Self {
            path
        }
    }

    fn create_file(path_buf: &PathBuf) -> Result<File, Error> {
        let file_result = File::create(&path_buf);
        match file_result {
            Ok(file) => Ok(file),
            Err(e) => Err(e),
        }
    }

    fn get_log_file(path_buf: &PathBuf) -> Result<File, Error> {
        let file = File::open(path_buf);
        match file {
            Ok(f) => Ok(f),
            Err(e) => Err(e)
        }
    }

    // Write log to the file
    pub fn write_log(&self, log_type: LogType) -> Result<(), Error> {
        todo!()
    }

    // Read logs from the file
    pub fn read_logs() {}

    // When the app is starting time checks if the log file exists
    fn check_logs() {}
}