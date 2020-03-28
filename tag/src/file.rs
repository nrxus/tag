use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::{DateTime, Utc};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
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

    let builder = FileLogBuilder {
        pid,
        username,
        command_line,
        process_name,
        path,
    };

    let mut logs = vec![builder.clone().build(create_time, Activity::Create)];

    if modify {
        tempfile.write_all(b"tag").map_err(Error::Write)?;
        tempfile.flush().map_err(Error::Write)?;
        let update_time = Utc::now();
        logs.push(builder.clone().build(update_time, Activity::Modify));
    }

    // force the file to be deleted now
    std::mem::drop(tempfile);
    let delete_time = Utc::now();

    logs.push(builder.build(delete_time, Activity::Delete));

    Ok(logs)
}

#[derive(Debug)]
pub enum Error {
    Create(std::io::Error),
    ExecName(ExecNameError),
    Write(std::io::Error),
}

#[derive(Clone)]
struct FileLogBuilder {
    username: String,
    pid: u32,
    command_line: String,
    process_name: String,
    path: PathBuf,
}

impl FileLogBuilder {
    pub fn build(self, time: DateTime<Utc>, activity: Activity) -> Log {
        Log {
            time,
            activity: match activity {
                Activity::Create => ActivityLog::FileCreated { path: self.path },
                Activity::Modify => ActivityLog::FileModified { path: self.path },
                Activity::Delete => ActivityLog::FileDeleted { path: self.path },
            },
            username: self.username,
            pid: self.pid,
            command_line: self.command_line,
            process_name: self.process_name,
        }
    }
}

#[derive(Clone, Copy)]
enum Activity {
    Create,
    Modify,
    Delete,
}
