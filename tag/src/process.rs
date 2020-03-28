use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::Utc;
use nix::unistd;

pub fn process(_: bool) -> Result<Vec<Log>, Error> {
    let command_line = std::env::args().collect::<Vec<String>>().join(" ");

    let fork = unistd::fork().map_err(|e| Error::WaitChild(assume_errno(e)))?;

    match fork {
        unistd::ForkResult::Child => std::process::exit(0),
        unistd::ForkResult::Parent { child } => {
            let _ = nix::sys::wait::waitpid(child, None)
                .map_err(|e| Error::WaitChild(assume_errno(e)))?;

            let log = Log {
                command_line,
                time: Utc::now(),
                username: whoami::username(),
                pid: std::process::id(),
                process_name: current_process_name().map_err(Error::ExecName)?,
                activity: ActivityLog::ProcessFork {
                    child_pid: i32::from(child) as u32,
                },
            };

            Ok(vec![log])
        }
    }
}

fn assume_errno(error: nix::Error) -> nix::errno::Errno {
    match error {
        nix::Error::Sys(e) => e,
        nix::Error::InvalidPath | nix::Error::InvalidUtf8 | nix::Error::UnsupportedOperation => {
            unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Fork(nix::errno::Errno),
    WaitChild(nix::errno::Errno),
    ExecName(ExecNameError),
}
