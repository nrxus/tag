# Telemetry Activy Generator (TAG) CLI

`tag-cli` is a CLI app to generate telemetry activity such as:

* Forking a process
* Exec'ing a process
* File Creation
* File Modification
* File Deletion
* Network Connection

After a succesful run, `tag-cli` will create a file containing
information about the generated activity.

Note that `tag-cli` is built explicitely to only support `unix`
systems.

## How to use

After tag has been succesfully installed (e.g., `cargo build
--release`), you can get its information by executing the help
command:

`path/to/tag-cli --help`

```
tag-cli 0.1.0
(T)elemetry (A)ctivity (G)enerator CLI

TAG provides subcommands to generate telemetry activity and generates
reports based on the activity generated.

TAG's purpose is to generate test activy and data to catch any
regressions in your Endpoint Detection and Response (EDR) agents.

TAG is capable of generating the following types of activities: file,
fork, and network. Each of these is its own subcommand. Refer to
their individual help texts for more information.

USAGE:
    tag-cli <SUBCOMMAND>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


SUBCOMMANDS:
    file        Generates file activities in a given path
    help        Prints this message or the help of the given subcommand(s)
    network     Generates a network activity
    playbook    Generates activity based on a given playbook
    fork        Generates a fork activity
```

For simple activities you can use the `file`, `network` or `fork`
subcommands to generate activity in a "one-shot". Note that this may
still trigger multiple activities, (e.g. `file` triggers both a create
file and a delete file activity).

For more interesting tests, you may use the `playbook` subcommand
which accepts a path to a yaml file with a list of activities such as:

```yaml
#
# activity_type can be:
# * file
# * fork
# * network
# these match to the tag-cli subcommands
#
- activity_type: file      # triggers file activities
  extension: txt           # required
  path: cli/               # required
  modify: true             # optional - defaults to false
- activity_type: network   # triggers a network activity
- activity_type: fork      # triggers a fork activity
  exec: true               # optional - defaults to false
```

Note that the activity types and its parameters match to the other
subcommands in `tag-cli`.

A created log for such a playbook would look like:

```yaml
---
- activity_type: file_created
  path: /home/andres/workspace/tag/cli/.tmpDh7UCw.txt
  time: "2020-03-28T20:47:26.838849841Z"
  username: andres
  pid: 20083
  command_line: target/debug/tag-cli playbook -p playbook.tag.yaml
  process_name: tag-cli
- activity_type: file_modified
  path: /home/andres/workspace/tag/cli/.tmpDh7UCw.txt
  time: "2020-03-28T20:47:26.838890004Z"
  username: andres
  pid: 20083
  command_line: target/debug/tag-cli playbook -p playbook.tag.yaml
  process_name: tag-cli
- activity_type: file_deleted
  path: /home/andres/workspace/tag/cli/.tmpDh7UCw.txt
  time: "2020-03-28T20:47:26.838935418Z"
  username: andres
  pid: 20083
  command_line: target/debug/tag-cli playbook -p playbook.tag.yaml
  process_name: tag-cli
- activity_type: network
  destination: "216.58.193.78:80"
  source: "192.168.0.104:49084"
  protocol: TCP
  bytes_sent: 4
  time: "2020-03-28T20:47:26.855191049Z"
  username: andres
  pid: 20083
  command_line: target/debug/tag-cli playbook -p playbook.tag.yaml
  process_name: tag-cli
- activity_type: fork
  child_pid: 20085
  time: "2020-03-28T20:47:26.855513015Z"
  username: andres
  pid: 20083
  command_line: target/debug/tag-cli playbook -p playbook.tag.yaml
  process_name: tag-cli
- activity_type: exec
  parent_pid: 20083
  time: "2020-03-28T20:47:26.856952077Z"
  username: andres
  pid: 20085
  command_line: "/usr/bin/printf ''"
  process_name: printf

```


## How it works

`tag-cli` uses a (for now internal) crate called `tag` for all the
heavy lifting of triggering activities. This separation makes `tag`
itself extendable to other forms of clients in the future.

`tag` exposes 3 functions, `file`, `network`, and `fork`. All of these
functions output one or more `Log` structs or an error if one
occured. `Log` has all the relevant data for the triggered activity
and it is ready to be serialized into whatever format a client might
need. Note that the timestamp on the various activities are all
gathered immediately after the activity has been triggered so a delay
is expected.

`file` uses named temporary files to ensure that the file is deleted
when the file goes out of scope. This makes sure that artifacts
created simply for activity triggering are cleaned up. If the file
needs to be modified a small number of bytes are written into it prior
to deletion.

`network` uses a tcp stream to connect into "google.com:80" and sends
a couple of bytes over. This function does not attempt to read a
response. If data cannot be sent over within one second then a timeout
error is bubbled up. The timestamp is collected after the data has
been succesfully sent (we do not care if the response itself is
succesful). A low-level `TcpStream` is used rather than a more high
level framework such as `reqwest` so that we may easily gather the
port of the local socket making the connection. An open question here
is whether `google.com` is a reliable enough server to hit when making
network connections, especially if we ever expand `tag` to allow for
concurrent tasks. In the future using a server we "own" might be a
more reliable way to trigger this network activity.

`fork` uses a wrapper around [fork(2)] to create a child process that
may optionally run an `exec` to replace the child process image. If no
`exec` is being run the child process immediately exits with a code of
0 to mark succcess. Otherwise, the child process runs `printf` with an
empty argument. This was chosen as it is the author's belief that
`printf` should exist in most `unix` systems. If this assumption is
proven incorrect or we need to support a `unix` system without
`printf` pre-installed, we would need to think of a separate common
command or ship our own no-op binary. `printf` also has the advantage
of printing nothing if an empty argument is passed, unlike `echo`
which by default adds a new line. The parent process, in turn, will
get the fork activity timestamp immediately after the succesful
fork. It will then wait for the child process to exit to avoid leaving
a zombie process. It will bubble up any non-succesful exits from the
child process. If the user asked for an `exec` after the fork, the
parent process will get the exec activity timestamp right after the
child has exited. The parent process is the one in charge of returning
the log data back to the client.

[fork(2)]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/fork.html

## TODOs:

Currently `tag` bubbles up its errors back to `tag-cli`, and then
`tag-cli` simply calls `expect` on all of its `Result`s. While this is
*fine* for a proof of concept CLI, it is far from the ideal UX. Work
needs to be done in the CLI to properly handle this errros and display
helpful messages to the user.

Look at the crate `clap` once v3 is released as it might help make the
CLI commands a bit cleaner without much work.

Tests, Tests, Tests. Currently neither the CLI app nor the `tag`
library itself contain any tests. Since most of the code is fairly
declarative (not much logic) I don't think we need many unit tests but
some high level tests to verify that is "generally" does the right
thing for all the activities is needed.
