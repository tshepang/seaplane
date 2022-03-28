With no args:

```console
$ seaplane metadata
? 2
seaplane-metadata [PKGVER]
Operate on metadata key-value pairs using the Global Data Coordination API

USAGE:
    seaplane metadata [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    delete    Delete one or more metadata key-value pairs [aliases: del, remove, rm]
    get       Get one or more metadata key-value pairs [aliases: show]
    help      Print this message or the help of the given subcommand(s)
    list      List one or more metadata key-value pairs [aliases: ls]
    set       Set a metadata key-value pair [aliases: put]

```

The short help:

```console
$ seaplane metadata -h
seaplane-metadata [PKGVER]
Operate on metadata key-value pairs using the Global Data Coordination API

USAGE:
    seaplane metadata [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    delete    Delete one or more metadata key-value pairs [aliases: del, remove, rm]
    get       Get one or more metadata key-value pairs [aliases: show]
    help      Print this message or the help of the given subcommand(s)
    list      List one or more metadata key-value pairs [aliases: ls]
    set       Set a metadata key-value pair [aliases: put]

```

The long help:

```console
$ seaplane metadata --help
seaplane-metadata [PKGVER]
Operate on metadata key-value pairs using the Global Data Coordination API

USAGE:
    seaplane metadata [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with your account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

        --format <FORMAT>
            Change the output format
            
            [default: table]
            [possible values: table, json]

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

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

SUBCOMMANDS:
    delete
            Delete one or more metadata key-value pairs [aliases: del, remove, rm]
    get
            Get one or more metadata key-value pairs [aliases: show]
    help
            Print this message or the help of the given subcommand(s)
    list
            List one or more metadata key-value pairs [aliases: ls]
    set
            Set a metadata key-value pair [aliases: put]

```
