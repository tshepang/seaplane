Without any arguments

```console
$ seaplane
? 2
seaplane [..]
Seaplane IO, Inc.

USAGE:
    seaplane[EXE] [OPTIONS] <SUBCOMMAND>

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
    account             Operate on Seaplane account details, including access tokens [aliases: acct]
    flight              Operate on local Flight Plans which define "Flights" (logical containers), and are then referenced by Formations
    formation           Operate on local Formations Plans and remote Formation Instances of those Plans
    help                Print this message or the help of the given subcommand(s)
    init                Create the Seaplane directory structure at the appropriate locations
    license             Print license information
    locks               Operate on the Locks API [aliases: l]
    metadata            Operate on metadata key-value pairs using the Global Data Coordination API [aliases: meta, md]
    restrict            Restrict the placement of data for Global Data Coordination API
    shell-completion    Generate shell completion script files for seaplane

```
