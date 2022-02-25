Be default, `seaplane flight` will display the help text:

```console
$ seaplane flight
? 2
seaplane-flight [PKGVER]
Operate on Seaplane Flights (locial containers), which are the core component of Formations

USAGE:
    seaplane flight [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --color <COLOR>    Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help             Print help information
        --no-color         Do not color output (alias for --color=never)
    -q, --quiet            Suppress output at a specific level and below
    -v, --verbose          Display more verbose output
    -V, --version          Print version information

SUBCOMMANDS:
    copy        Copy a Flight definition [aliases: clone]
    create      Create a new Flight definition [aliases: add]
    delete      Delete a Flight definition [aliases: del, remove, rm]
    edit        Edit a Flight definition [aliases: clone]
    help        Print this message or the help of the given subcommand(s)
    list        List the current Flight definitions [aliases: ls]
    template    Generate a new template skeleton for a Flight definition

```
