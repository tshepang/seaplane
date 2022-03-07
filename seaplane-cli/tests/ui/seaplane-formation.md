Be default, `seaplane formation` will display the help text:

```console
$ seaplane formation
? 2
seaplane-formation [PKGVER]
Operate on Seaplane Formations

USAGE:
    seaplane formation [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --color <COLOR>    Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help             Print help information
        --no-color         Do not color output (alias for --color=never)
    -q, --quiet            Suppress output at a specific level and below
    -v, --verbose          Display more verbose output
    -V, --version          Print version information

SUBCOMMANDS:
    configuration            Operate on Seaplane Formation Configurations [aliases: cfg]
    container-statistics     Display statistics about the underlying physical container instances [aliases: container-stats]
    create                   Create a Seaplane Formation [aliases: add]
    delete                   Delete a Seaplane Formation [aliases: del, remove, rm]
    help                     Print this message or the help of the given subcommand(s)
    list                     List your Seaplane Formations [aliases: ls]
    stop                     Stop all instances of a Formation
    template                 Generate a template skeleton of a Formation
    traffic-configuration    Control how traffic balances between formation configurations

```
