mod log;
mod network;
mod process;

pub use log::Log;
pub use process::{process};
pub use network::{network, NetworkError};

use chrono::offset::Utc;
use std::path::{Path, PathBuf};

pub fn file(_: &Path, _: &'_ str, _: bool) -> Result<Vec<Log>, ExecNameError> {
    Ok(vec![Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: current_process_name()?,
        activity: log::ActivityLog::FileCreated {
            path: PathBuf::new(),
        },
    }])
}



fn current_process_name() -> Result<String, ExecNameError> {
    Ok(std::env::current_exe()
        .map_err(ExecNameError::GetExecutable)?
        .file_name()
        .unwrap() // OK; all executables should have a file name (last path segment should not be '..')
        .to_str()
        .ok_or_else(|| ExecNameError::NonUTF8ExecName)?
        .to_string())
}

#[derive(Debug)]
pub enum ExecNameError {
    GetExecutable(std::io::Error),
    NonUTF8ExecName,
}
