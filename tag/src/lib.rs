#![cfg(unix)]

mod log;

pub mod file;
pub mod fork;
pub mod network;

pub use file::file;
pub use fork::fork;
pub use log::{ActivityLog, Log};
pub use network::network;

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
