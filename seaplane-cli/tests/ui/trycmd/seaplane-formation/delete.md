```console
$ seaplane formation delete -h
seaplane-formation-delete [..]
Deletes local Formation Plans and/or remote Formation Instances

USAGE:
    seaplane formation delete <NAME|ID> [OPTIONS]
    seaplane formation delete <NAME|ID> --no-remote [OPTIONS]

ARGS:
    <NAME|ID>    The name or ID of the Formation to remove, must be unambiguous

OPTIONS:
    -a, --all                 Delete all matching Formations even when the name or ID is ambiguous or a partial match
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -f, --force               Delete this Formation even if there are remote instances In Flight (active), which will effectively stop all remote instances of this Formation
    -h, --help                Print help information
        --local               Delete local Formation Definitions (this is set by the default, use --no-local to skip)
        --no-color            Do not color output (alias for --color=never)
        --no-local            DO NOT delete local Formation Definitions
        --no-remote           DO NOT delete remote Formation Instances (this is set by the default, use --remote to remove them)
    -q, --quiet               Suppress output at a specific level and below
    -r, --recursive           Recursively delete all local definitions associated with this Formation
        --remote              Delete remote Formation Instances (this is set by default, use --no-remote to skip)
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

```console
$ seaplane formation delete --help
seaplane-formation-delete [..]
Deletes local Formation Plans and/or remote Formation Instances

USAGE:
    seaplane formation delete <NAME|ID> [OPTIONS]
    seaplane formation delete <NAME|ID> --no-remote [OPTIONS]

ARGS:
    <NAME|ID>
            The name or ID of the Formation to remove, must be unambiguous

OPTIONS:
    -a, --all
            Delete all matching Formations even when the name or ID is ambiguous or a partial match

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

    -f, --force
            Delete this Formation even if there are remote instances In Flight (active), which will effectively stop all remote instances of this Formation

    -h, --help
            Print help information

        --local
            Delete local Formation Definitions (this is set by the default, use --no-local to skip)

        --no-color
            Do not color output (alias for --color=never)

        --no-local
            DO NOT delete local Formation Definitions

        --no-remote
            DO NOT delete remote Formation Instances (this is set by the default, use --remote to remove them)

    -q, --quiet
            Suppress output at a specific level and below
            
            More uses suppresses higher levels of output
                -q:   Only display WARN messages and above
                -qq:  Only display ERROR messages
                -qqq: Suppress all output

    -r, --recursive
            Recursively delete all local definitions associated with this Formation

        --remote
            Delete remote Formation Instances (this is set by default, use --no-remote to skip)

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
