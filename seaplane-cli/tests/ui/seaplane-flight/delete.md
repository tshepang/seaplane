
With no additional arguments, an error is displayed about a missing required argument.

```console
$ seaplane flight delete
? 2
error: The following required arguments were not provided:
    <NAME|ID>

USAGE:
    seaplane flight delete <NAME|ID> [OPTIONS]

For more information try --help

```

The short help message with `-h`:

```console
$ seaplane flight delete -h
seaplane-flight-delete [PKGVER]
Delete a Flight definition

USAGE:
    seaplane flight delete <NAME|ID> [OPTIONS]

ARGS:
    <NAME|ID>    The name or hash of the Flight to remove, must be unambiguous

OPTIONS:
    -a, --all              Delete all matching Flights even when FLIGHT is ambiguous
        --color <COLOR>    Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --force            Delete this Flight even if referenced by a Formation (removes any references in Formations), or deletes ALL Flights referencedd by <FLIGHT> even if ambiguous
    -h, --help             Print help information
        --no-color         Do not color output (alias for --color=never)
    -q, --quiet            Suppress output at a specific level and below
    -v, --verbose          Display more verbose output
    -V, --version          Print version information
    -x, --exact            the given FLIGHT must be an exact match

```

The long help message with `--help`:

```console
$ seaplane flight delete --help
seaplane-flight-delete [PKGVER]
Delete a Flight definition

USAGE:
    seaplane flight delete <NAME|ID> [OPTIONS]

ARGS:
    <NAME|ID>
            The name or hash of the Flight to remove, must be unambiguous

OPTIONS:
    -a, --all
            Delete all matching Flights even when FLIGHT is ambiguous

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

        --force
            Delete this Flight even if referenced by a Formation (removes any references in Formations), or deletes ALL Flights referencedd by <FLIGHT> even if ambiguous

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

    -x, --exact
            the given FLIGHT must be an exact match

```
