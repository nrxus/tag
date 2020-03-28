use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::Utc;
use std::{io::Write, path::Path};
use tempfile;

pub fn file(path: &Path, file_type: &'_ str, modify: bool) -> Result<Vec<Log>, Error> {
    let username = whoami::username();
    let pid = std::process::id();
    let command_line = std::env::args().collect::<Vec<String>>().join(" ");
    let process_name = current_process_name().map_err(Error::ExecName)?;
    let mut suffix = String::from(".");
    suffix.push_str(file_type);

    let mut tempfile = tempfile::Builder::new()
        .suffix(&suffix)
        .tempfile_in(path)
        .map_err(Error::Create)?;

    let create_time = Utc::now();
    let path = tempfile.path().to_path_buf();

    let mut logs = vec![Log {
        pid,
        username: username.clone(),
        command_line: command_line.clone(),
        process_name: process_name.clone(),
        time: create_time,
        activity: ActivityLog::FileCreated { path: path.clone() },
    }];

    if modify {
        tempfile.write_all(b"tag").map_err(Error::Write)?;
        tempfile.flush().map_err(Error::Write)?;
        logs.push(Log {
            pid,
            username: username.clone(),
            command_line: command_line.clone(),
            process_name: process_name.clone(),
            time: Utc::now(),
            activity: ActivityLog::FileModified { path: path.clone() },
        })
    }

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
    Write(std::io::Error),
}
