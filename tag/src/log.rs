use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Log {
    #[serde(flatten)]
    pub activity: ActivityLog,
    pub time: DateTime<Utc>,
    pub username: String,
    pub pid: u16,
    pub command_line: String,
    pub process_name: String,
}

#[derive(Serialize)]
#[serde(tag = "activity_type")]
pub enum ActivityLog {
    Network {
        destination: String,
        source: String,
        protocol: String,
        data_size: usize,
    },
    ProcessFork {
        child_pid: u16,
    },
    ProcessExec {
        parent_pid: u16,
    },
    FileCreated {
        path: PathBuf,
    },
    FileModified {
        path: PathBuf,
    },
    FileDeleted {
        path: PathBuf,
    },
}
