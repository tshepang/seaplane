Without any arguments

```console
$ seaplane
? 2
seaplane [PKGVER]
Seaplane IO, Inc.

USAGE:
    seaplane [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --color <COLOR>    Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help             Print help information
        --no-color         Do not color output (alias for --color=never)
    -q, --quiet            Suppress output at a specific level and below
    -v, --verbose          Display more verbose output
    -V, --version          Print version information

SUBCOMMANDS:
    account             
    config              
    flight              Operate on Seaplane Flights (logical containers), which are the core component of Formations
    formation           Operate on Seaplane Formations
    help                Print this message or the help of the given subcommand(s)
    image               
    init                Create the Seaplane directory structure at the appropriate locations
    license             
    shell-completion    Generate shell completion script files for seaplane

```
