use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Log {
    #[serde(flatten)]
    pub activity: ActivityLog,
    pub time: DateTime<Utc>,
    pub username: String,
    pub pid: u32,
    pub command_line: String,
    pub process_name: String,
}

#[derive(Serialize)]
#[serde(tag = "activity_type", rename_all = "snake_case")]
pub enum ActivityLog {
    Network {
        destination: std::net::SocketAddr,
        source: std::net::SocketAddr,
        protocol: &'static str,
        bytes_sent: usize,
    },
    Fork {
        child_pid: u32,
    },
    Exec {
        parent_pid: u32,
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
