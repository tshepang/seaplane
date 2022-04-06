```console
$ seaplane formation delete -h
seaplane-formation-delete [PKGVER]
Delete a Seaplane Formation

USAGE:
    seaplane formation delete <NAME|ID> [OPTIONS]

ARGS:
    <NAME|ID>    The name or ID of the Formation to remove, must be unambiguous

OPTIONS:
    -a, --all                 Delete all matching Formations even when FORMATION is ambiguous
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -f, --force               Delete this Formation even if there are configurations In Flight (active), which will effectively stop all instances of this Formation
    -h, --help                Print help information
        --local               Delete local Formations (this is set by the default, use --no-local to skip)
        --no-color            Do not color output (alias for --color=never)
        --no-local            DO NOT delete local Formations
        --no-remote           DO NOT delete remote Formations (this is set by the default, use --remote to remove them)
    -q, --quiet               Suppress output at a specific level and below
    -r, --recursive           Recursively delete all local objects associated with this Formation
        --remote              Delete remote Formations (this is set by default, use --no-remote to skip)
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information
    -x, --exact               The given FORMATION must be an exact match

```

```console
$ seaplane formation delete --help
seaplane-formation-delete [PKGVER]
Delete a Seaplane Formation

USAGE:
    seaplane formation delete <NAME|ID> [OPTIONS]

ARGS:
    <NAME|ID>
            The name or ID of the Formation to remove, must be unambiguous

OPTIONS:
    -a, --all
            Delete all matching Formations even when FORMATION is ambiguous

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

    -f, --force
            Delete this Formation even if there are configurations In Flight (active), which will effectively stop all instances of this Formation

    -h, --help
            Print help information

        --local
            Delete local Formations (this is set by the default, use --no-local to skip)

        --no-color
            Do not color output (alias for --color=never)

        --no-local
            DO NOT delete local Formations

        --no-remote
            DO NOT delete remote Formations (this is set by the default, use --remote to remove them)

    -q, --quiet
            Suppress output at a specific level and below
            
            More uses suppresses higher levels of output
                -q:   Only display WARN messages and above
                -qq:  Only display ERROR messages
                -qqq: Suppress all output

    -r, --recursive
            Recursively delete all local objects associated with this Formation

        --remote
            Delete remote Formations (this is set by default, use --no-remote to skip)

    -S, --stateless
            Ignore local state files, do not read from or write to them

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

    -x, --exact
            The given FORMATION must be an exact match

```
