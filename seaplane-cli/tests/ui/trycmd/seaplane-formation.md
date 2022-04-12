Be default, `seaplane formation` will display the help text:

```console
$ seaplane formation
? 2
seaplane-formation [..]
Operate on local Formations Plans and remote Formation Instances of those Plans

USAGE:
    seaplane formation [OPTIONS] <SUBCOMMAND>

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
    delete          Deletes local Formation Plans and/or remote Formation Instances [aliases: del, remove, rm]
    fetch-remote    Fetch remote Formation Instances and create/synchronize local Plan definitions [aliases: fetch, sync, synchronize]
    help            Print this message or the help of the given subcommand(s)
    land            Land (Stop) all configurations of a remote Formation Instance [aliases: stop]
    launch          Start a local Formation Plan creating a remote Formation Instance [aliases: start]
    list            List all local Formation Plans [aliases: ls]
    plan            Create a Seaplane Formation [aliases: create, add]

```
