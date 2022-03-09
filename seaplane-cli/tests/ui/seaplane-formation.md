Be default, `seaplane formation` will display the help text:

```console
$ seaplane formation
? 2
seaplane-formation [PKGVER]
Operate on Seaplane Formations

USAGE:
    seaplane formation [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    create          Create a Seaplane Formation [aliases: add]
    delete          Delete a Seaplane Formation [aliases: del, remove, rm]
    fetch-remote    Fetch remote Formation definitions [aliases: fetch]
    help            Print this message or the help of the given subcommand(s)
    land            Land (Stop) all configurations of a Formation [aliases: stop]
    launch          Start all configurations of a Formation and evenly distribute traffic between them [aliases: start]
    list            List your Seaplane Formations [aliases: ls]

```
