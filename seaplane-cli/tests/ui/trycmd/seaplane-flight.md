Be default, `seaplane flight` will display the help text:

```console
$ seaplane flight
? 2
seaplane-flight [..]
Operate on local Flight Plans which define "Flights" (logical containers), and are then referenced by Formations

USAGE:
    seaplane flight [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    copy      Copy a local Flight Plan (optionally make changes to the copy) [aliases: clone]
    delete    Delete a local Flight Plan [aliases: del, remove, rm]
    edit      Edit a local Flight Plan
    help      Print this message or the help of the given subcommand(s)
    list      List all local Flight Plans [aliases: ls]
    plan      Make a new local Flight Plan that Formations can include and reference [aliases: create, add]

```
