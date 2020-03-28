#![cfg(unix)]

use serde::Deserialize;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// (T)elemetry (A)ctivity (G)enerator CLI
///
/// TAG provides subcommands to generate telemetry activity and
/// generates reports based on the activity generated.
///
/// TAG's purpose is to generate test activy and data to catch any
/// regressions in your Endpoint Detection and Response (EDR) agents.
///
/// TAG is capable of generating the following types of activities:
/// file, fork, and network. Each of these is its own
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
        /// Path to log file to create
        #[structopt(short, long, default_value = "./logs.tag.yaml")]
        log_path: PathBuf,
    },
    /// Generates a fork activity
    ///
    /// Forks the current process and optionally executes a new one
    Fork {
        /// Flag to execute a process as part of the fork
        #[structopt(short, long)]
        exec: bool,
        /// Path to log file to create
        #[structopt(short, long, default_value = "./logs.tag.yaml")]
        log_path: PathBuf,
    },
    /// Generates a network activity
    Network {
        /// Path to log file to create
        #[structopt(short, long, default_value = "./logs.tag.yaml")]
        log_path: PathBuf,
    },
    /// Generates activity based on a given playbook
    ///
    /// Playbooks are YAML files that detail a list of activities
    Playbook {
        /// Path to playbook file
        #[structopt(short, long)]
        playbook: PathBuf,
        /// Path to log file to create
        #[structopt(short, long, default_value = "./logs.tag.yaml")]
        log_path: PathBuf,
    },
}

#[derive(Deserialize)]
#[serde(tag = "activity_type", rename_all = "snake_case")]
enum Activity {
    File {
        #[serde(default)]
        modify: bool,
        path: PathBuf,
        extension: String,
    },
    Fork {
        #[serde(default)]
        exec: bool,
    },
    Network,
}

fn main() {
    let (activities, path) = match Command::from_args() {
        Command::File {
            modify,
            path,
            extension,
            log_path,
        } => (
            vec![Activity::File {
                modify,
                path,
                extension,
            }],
            log_path,
        ),
        Command::Fork { exec, log_path } => (vec![Activity::Fork { exec }], log_path),
        Command::Network { log_path } => (vec![Activity::Network], log_path),
        Command::Playbook { playbook, log_path } => {
            let playbook = File::open(playbook).expect("could not open playbook file");
            let activities: Vec<Activity> =
                serde_yaml::from_reader(&playbook).expect("failed to parse playbook");
            (activities, log_path)
        }
    };

    let logs: Vec<tag::Log> = activities
        .into_iter()
        .flat_map(|a| match a {
            Activity::File {
                modify,
                path,
                extension,
            } => tag::file(&path, &extension, modify).expect("failed to create file activity"),
            Activity::Fork { exec } => tag::fork(exec).expect("failed to create fork activity"),
            Activity::Network => tag::network()
                .map(|l| vec![l])
                .expect("failed to create network activity"),
        })
        .collect();

    let file = File::create(path).expect("could not log file for creation");
    serde_yaml::to_writer(file, &logs).expect("could not write to log file after creation");
}
