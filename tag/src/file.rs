use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::Utc;
use std::path::Path;
use tempfile;

pub fn file(path: &Path, suffix: &'_ str, _: bool) -> Result<Vec<Log>, Error> {
    let username = whoami::username();
    let pid = std::process::id();
    let command_line = std::env::args().collect::<Vec<String>>().join(" ");
    let process_name = current_process_name().map_err(Error::ExecName)?;

    let tempfile = tempfile::Builder::new()
        .suffix(suffix)
        .tempfile_in(path)
        .map_err(Error::Create)?;

    let create_time = Utc::now();
    let path = tempfile.path().to_path_buf();

    let mut logs = vec![Log {
        username: username.clone(),
        pid,
        command_line: command_line.clone(),
        process_name: process_name.clone(),
        time: create_time,
        activity: ActivityLog::FileCreated { path: path.clone() },
    }];

    // force the file to be deleted now
    std::mem::drop(tempfile);

    logs.push(Log {
        username,
        pid,
        command_line,
        process_name,
        time: Utc::now(),
        activity: ActivityLog::FileDeleted { path },
    });

    Ok(logs)
}

#[derive(Debug)]
pub enum Error {
    Create(std::io::Error),
    ExecName(ExecNameError),
}
