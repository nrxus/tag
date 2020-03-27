mod log;

pub use log::Log;

use chrono::offset::Utc;
use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

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

pub fn process(_: bool) -> Result<Vec<Log>, ExecNameError> {
    Ok(vec![Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: current_process_name()?,
        activity: log::ActivityLog::ProcessFork { child_pid: 123 },
    }])
}

pub fn network() -> Result<Log, NetworkError> {
    let mut stream =
        std::net::TcpStream::connect("google.com:80").map_err(NetworkError::Connect)?;
    stream
        .set_write_timeout(Some(Duration::from_secs(1)))
        .map_err(NetworkError::SetTimeout)?;

    let time = Utc::now();

    // some random data
    let buffer = [1, 1, 1, 4];

    stream.write(&buffer).map_err(NetworkError::Write)?;

    let command_line = std::env::args().collect::<Vec<String>>().join(" ");

    let log = Log {
        time,
        command_line,
        username: whoami::username(),
        pid: std::process::id(),
        process_name: current_process_name().map_err(NetworkError::ExecName)?,
        activity: log::ActivityLog::Network {
            destination: stream.peer_addr().map_err(NetworkError::RemoteAddr)?,
            source: stream.local_addr().map_err(NetworkError::LocalAddr)?,
            protocol: "TCP",
            bytes_sent: buffer.len(),
        },
    };

    Ok(log)
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
pub enum NetworkError {
    Connect(std::io::Error),
    SetTimeout(std::io::Error),
    Write(std::io::Error),
    ExecName(ExecNameError),
    RemoteAddr(std::io::Error),
    LocalAddr(std::io::Error),
}

#[derive(Debug)]
pub enum ExecNameError {
    GetExecutable(std::io::Error),
    NonUTF8ExecName,
}
