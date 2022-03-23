Without any arguments

```console
$ seaplane
? 2
seaplane [PKGVER]
Seaplane IO, Inc.

USAGE:
    seaplane [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    account             Operate on your Seaplane account, including access tokens [aliases: acct]
    flight              Operate on Seaplane Flights (logical containers), which are the core component of Formations
    formation           Operate on Seaplane Formations
    help                Print this message or the help of the given subcommand(s)
    init                Create the Seaplane directory structure at the appropriate locations
    key-value           Operate on key-value pairs using the Global Data Consensus API [aliases: kv]
    license             Print license information
    shell-completion    Generate shell completion script files for seaplane

```
