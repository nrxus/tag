use crate::{
    current_process_name,
    log::{ActivityLog, Log},
    ExecNameError,
};
use chrono::Utc;
use nix::{sys, unistd};
use std::ffi::CString;

pub fn process(exec: bool) -> Result<Vec<Log>, Error> {
    let command_line = std::env::args().collect::<Vec<String>>().join(" ");

    let fork = unistd::fork().map_err(|e| Error::WaitChild(assume_errno(e)))?;

    match fork {
        unistd::ForkResult::Child => {
            if exec {
                let ls = CString::new("/usr/bin/printf").expect("CString::new failed");

                unistd::execv(
                    &ls,
                    &[
                        &&ls,
                        &&CString::new("").unwrap(),
                    ],
                )
                .expect("failed to execute");
                unreachable!();
            } else {
                std::process::exit(0)
            }
        }
        unistd::ForkResult::Parent { child } => {
            let child_satus =
                sys::wait::waitpid(child, None).map_err(|e| Error::WaitChild(assume_errno(e)))?;

            let username = whoami::username();
            let parent_pid = std::process::id();
            let child_pid = i32::from(child) as u32;

            let mut logs = vec![Log {
                command_line,
                username: username.clone(),
                time: Utc::now(),
                pid: parent_pid,
                process_name: current_process_name().map_err(Error::ExecName)?,
                activity: ActivityLog::ProcessFork { child_pid },
            }];

            if exec {
                match child_satus {
                    sys::wait::WaitStatus::Exited(_, 0) => logs.push(Log {
                        username,
                        command_line: "/usr/bin/printf ''".into(),
                        time: Utc::now(),
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
