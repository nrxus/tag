use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::Utc;
use std::{io::Write, time::Duration};

pub fn network() -> Result<Log, Error> {
    let mut stream = std::net::TcpStream::connect("google.com:80").map_err(Error::Connect)?;
    stream
        .set_write_timeout(Some(Duration::from_secs(1)))
        .map_err(Error::SetTimeout)?;

    // some random data
    let buffer = [1, 1, 1, 4];

    stream.write(&buffer).map_err(Error::Write)?;

    // get time right after we wrote the data
    let time = Utc::now();
    let command_line = std::env::args().collect::<Vec<String>>().join(" ");

    let log = Log {
        time,
        command_line,
        username: whoami::username(),
        pid: std::process::id(),
        process_name: current_process_name().map_err(Error::ExecName)?,
        activity: ActivityLog::Network {
            destination: stream.peer_addr().map_err(Error::RemoteAddr)?,
            source: stream.local_addr().map_err(Error::LocalAddr)?,
            protocol: "TCP",
            bytes_sent: buffer.len(),
        },
    };

    Ok(log)
}

#[derive(Debug)]
pub enum Error {
    Connect(std::io::Error),
    SetTimeout(std::io::Error),
    Write(std::io::Error),
    ExecName(ExecNameError),
    RemoteAddr(std::io::Error),
    LocalAddr(std::io::Error),
}
