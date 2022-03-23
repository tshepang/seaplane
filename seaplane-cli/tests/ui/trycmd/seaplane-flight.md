Be default, `seaplane flight` will display the help text:

```console
$ seaplane flight
? 2
seaplane-flight [PKGVER]
Operate on Seaplane Flights (logical containers), which are the core component of Formations

USAGE:
    seaplane flight [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    copy      Copy a Flight definition [aliases: clone]
    create    Create a new Flight definition [aliases: add]
    delete    Delete a Flight definition [aliases: del, remove, rm]
    edit      Edit a Flight definition
    help      Print this message or the help of the given subcommand(s)
    list      List the current Flight definitions [aliases: ls]

```
