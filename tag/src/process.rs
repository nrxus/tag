use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::Utc;
use nix::{sys, unistd};
use std::ffi::CString;

pub fn process(exec: bool) -> Result<Vec<Log>, Error> {
    let fork = unistd::fork().map_err(|e| Error::WaitChild(assume_errno(e)))?;

    match fork {
        unistd::ForkResult::Child => {
            if exec {
                let printf = CString::new("/usr/bin/printf").unwrap(); // OK; internal bug if it fails
                let param = CString::new("").unwrap(); // OK; internal bug if it fails

                unistd::execv(&printf, &[&&printf, &&param]).expect("failed to execute");

                unreachable!();
            } else {
                std::process::exit(0)
            }
        }
        unistd::ForkResult::Parent { child } => {
            // time right after we fork but before we wait for the child
            let fork_time = Utc::now();

            let child_status =
                sys::wait::waitpid(child, None).map_err(|e| Error::WaitChild(assume_errno(e)))?;

            let child_info = if exec {
                Some((child_status, Utc::now()))
            } else {
                None
            };

            let username = whoami::username();
            let parent_pid = std::process::id();
            let child_pid = i32::from(child) as u32;

            let mut logs = vec![Log {
                command_line: std::env::args().collect::<Vec<String>>().join(" "),
                username: username.clone(),
                time: fork_time,
                pid: parent_pid,
                process_name: current_process_name().map_err(Error::ExecName)?,
                activity: ActivityLog::ProcessFork { child_pid },
            }];

            if let Some((status, time)) = child_info {
                match status {
                    sys::wait::WaitStatus::Exited(_, 0) => logs.push(Log {
                        username,
                        time,
                        command_line: "/usr/bin/printf ''".into(),
                        pid: child_pid,
                        process_name: "printf".into(),
                        activity: ActivityLog::ProcessExec { parent_pid },
                    }),
                    _ => return Err(Error::ChildExit),
                }
            }

            Ok(logs)
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
    ChildExit,
}
