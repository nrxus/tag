#![cfg(unix)]

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
/// (T)elemetry (A)ctivity (G)enerator
///
/// TAG provides subcommands to generate telemetry activity and
/// generates reports based on the activity generated.
///
/// TAG's purpose is to generate test activy and data to catch any
/// regressions in your Endpoint Detection and Response (EDR) agents.
///
/// TAG is capable of generating the following types of activities:
/// file, process, and network. Each of these is its own
/// subcommand. Refer to their individual help texts for more
/// information.
enum Command {
    /// Generates file activities in a given path
    ///
    /// Given a specified path and extension, this subcommand will
    /// generate activity that creates a file, optionally modifies it,
    /// and then deletes it.
    File {
        /// Flag to modify the file prior to deletion
        #[structopt(short, long)]
        modify: bool,
        /// Directory path where to create the file
        #[structopt(short, long)]
        path: PathBuf,
        /// Extension for the file to create
        #[structopt(short, long)]
        extension: String,
    },
    /// Generates process activities
    ///
    /// Forks the current process and optionally executes a new one
    Process {
        /// Flag to execute a new process as part of the fork
        #[structopt(short, long)]
        exec: bool,
    },
    /// Generates a network activity
    Network,
}

fn main() {
    let opt = Command::from_args();
    println!("{:?}", opt);
}
