mod log;

pub use log::Log;

use chrono::offset::Utc;
use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

pub fn file(_: &Path, _: &'_ str, _: bool) -> Vec<Log> {
    vec![Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: current_process_name(),
        activity: log::ActivityLog::FileCreated {
            path: PathBuf::new(),
        },
    }]
}

pub fn process(_: bool) -> Vec<Log> {
    vec![Log {
        time: Utc::now(),
        username: "foobar".to_string(),
        pid: 1234,
        command_line: "foobar".to_string(),
        process_name: current_process_name(),
        activity: log::ActivityLog::ProcessFork { child_pid: 123 },
    }]
}

pub fn network() -> Log {
    let mut stream = std::net::TcpStream::connect("google.com:80").expect("could not connect");
    stream
        .set_write_timeout(Some(Duration::from_secs(1)))
        .expect("failed to set a write timeout");

    let time = Utc::now();

    // some random data
    let buffer = [1, 1, 1, 4];

    stream.write(&buffer).expect("failed to write data");

    let command_line = std::env::args().collect::<Vec<String>>().join(" ");

    Log {
        time,
        command_line,
        username: whoami::username(),
        pid: std::process::id(),
        process_name: current_process_name(),
        activity: log::ActivityLog::Network {
            destination: stream.peer_addr().expect("could not get remote address"),
            source: stream.local_addr().expect("could not get local address"),
            protocol: "TCP",
            data_size: buffer.len(),
        },
    }
}

fn current_process_name() -> String {
    std::env::current_exe()
        .expect("could not get current executable")
        .file_name()
        .expect("could not get executable file name")
        .to_str()
        .expect("process name not a valid UTF-8 name")
        .to_string()
}
