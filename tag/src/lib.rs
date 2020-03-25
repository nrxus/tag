mod log;

pub use log::Log;

use chrono::offset::Utc;
use std::path::{Path, PathBuf};

pub fn file(_: &Path, _: &'_ str, _: bool) -> Vec<Log> {
    vec![Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: "foobar".to_string(),
        activity: log::ActivityLog::File {
            path: PathBuf::new(),
            file_activity: log::FileActivity::Create,
        },
    }]
}

pub fn process(_: bool) -> Vec<Log> {
    vec![Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: "foobar".to_string(),
        activity: log::ActivityLog::ProcessFork { child_pid: 123 },
    }]
}

pub fn network() -> Log {
    Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: "foobar".to_string(),
        activity: log::ActivityLog::ProcessFork { child_pid: 123 },
    }
}
