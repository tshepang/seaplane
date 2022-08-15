Short help message with `-h`:

```console
$ seaplane init -h
seaplane[EXE]-init [..]
Create the Seaplane directory structure at the appropriate locations

USAGE:
    seaplane[EXE] init [OPTIONS]

OPTIONS:
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --force               Force create the files and directories (DANGER: will overwrite existing files)
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
        --overwrite <ITEM>    Overwrite select files or directories (DANGER: will overwrite existing data) (supports comma separated list, or multiple uses) [possible values: all, formations, flights, config]
    -q, --quiet               Suppress output at a specific level and below
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

The long help message with `--help`:

```console
$ seaplane init --help
seaplane[EXE]-init [..]
Create the Seaplane directory structure at the appropriate locations

USAGE:
    seaplane[EXE] init [OPTIONS]

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

        --force
            Force create the files and directories (DANGER: will overwrite existing files)
            
            Using --force is the same as using --overwrite=all

    -h, --help
            Print help information

        --no-color
            Do not color output (alias for --color=never)

        --overwrite <ITEM>
            Overwrite select files or directories (DANGER: will overwrite existing data)
            
            Using --overwrite=all is the same as using --force
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [possible values: all, formations, flights, config]

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
