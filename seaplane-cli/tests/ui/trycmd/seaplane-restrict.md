With no args:

```console
$ seaplane restrict
? 2
seaplane[EXE]-restrict [..]
Restrict the placement of data for Global Data Coordination API

USAGE:
    seaplane[EXE] restrict [OPTIONS] <SUBCOMMAND>

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
    delete    Delete a restriction on directory [aliases: del, remove, rm]
    get       Retrieve information about a directory restriction [aliases: show]
    help      Print this message or the help of the given subcommand(s)
    list      List restrictions in an API, or across all APIs [aliases: ls]
    set       Set a restriction [aliases: put]

```

The short help:

```console
$ seaplane restrict -h
seaplane[EXE]-restrict [..]
Restrict the placement of data for Global Data Coordination API

USAGE:
    seaplane[EXE] restrict [OPTIONS] <SUBCOMMAND>

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
    delete    Delete a restriction on directory [aliases: del, remove, rm]
    get       Retrieve information about a directory restriction [aliases: show]
    help      Print this message or the help of the given subcommand(s)
    list      List restrictions in an API, or across all APIs [aliases: ls]
    set       Set a restriction [aliases: put]

```

The long help:

```console
$ seaplane restrict --help
seaplane[EXE]-restrict [..]
Restrict the placement of data for Global Data Coordination API

USAGE:
    seaplane[EXE] restrict [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with a Seaplane account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -h, --help
            Print help information

        --no-color
            Do not color output (alias for --color=never)

    -q, --quiet
            Suppress output at a specific level and below
            
            More uses suppresses higher levels of output
                -q:   Only display WARN messages and above
                -qq:  Only display ERROR messages
                -qqq: Suppress all output

    -S, --stateless
            Ignore local state files, do not read from or write to them

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

SUBCOMMANDS:
    delete
            Delete a restriction on directory [aliases: del, remove, rm]
    get
            Retrieve information about a directory restriction [aliases: show]
    help
            Print this message or the help of the given subcommand(s)
    list
            List restrictions in an API, or across all APIs [aliases: ls]
    set
            Set a restriction [aliases: put]

```
