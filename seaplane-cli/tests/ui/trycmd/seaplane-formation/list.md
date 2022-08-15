```console
$ seaplane formation list -h
seaplane[EXE]-formation-list [..]
List all local Formation Plans

USAGE:
    seaplane[EXE] formation list [OPTIONS]

OPTIONS:
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -F, --fetch               Fetch remote Formation Instances and create/synchronize with local Plan Definitions prior to listing (by default only local Plans are displayed) [aliases: sync, synchronize]
        --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

```console
$ seaplane formation list --help
seaplane[EXE]-formation-list [..]
List all local Formation Plans

This command will display the status and number of configurations for each of your Formation
Plans. The Formations displayed come from the local database of known Formations. You may wish
to update the local database with Remote Formation Instances as well by either first running:

[..]

OR including `--fetch` such as:

[..]

After which your local database of Formation and Flight Plans will contain all remote Formation
Instances and their configurations as well.

USAGE:
    seaplane[EXE] formation list [OPTIONS]

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

    -F, --fetch
            Fetch remote Formation Instances and create/synchronize with local Plan Definitions prior to listing (by default only local Plans are displayed)
            
            [aliases: sync, synchronize]

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

    -S, --stateless
            Ignore local state files, do not read from or write to them

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

```
