Short help:

```console
$ seaplane metadata list -h
seaplane[EXE]-metadata-list [..]
List one or more metadata key-value pairs

USAGE:
    seaplane metadata list <DIR> [OPTIONS]

ARGS:
    <DIR>    The root directory of the metadata key-value pairs to list

OPTIONS:
    -A, --api-key <STRING>              The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
    -B, --base64                        The keys/values are already encoded in URL safe Base64
        --color <COLOR>                 Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -D, --decode                        Decode the keys and values before printing them
        --decode-safe                   Decode the keys and values in a terminal-friendly way
    -f, --from <KEY>                    Only print metadata key-value pairs after this key (note: if this key has a value it will be included in the results)
        --format <FORMAT>               Change the output format [default: table] [possible values: table, json]
    -h, --help                          Print help information
    -H, --no-header                     Omit the 'KEY' or 'VALUE' heading when printing with `--format=table` [aliases: no-heading, no-headers]
        --keys-width-limit <LIMIT>      Limit the width of the keys when using `--format=table` (0 means unlimited)
        --no-color                      Do not color output (alias for --color=never)
        --no-decode                     Print keys and values without decoding them
        --only-keys                     Only print the key [aliases: only-key]
        --only-values                   Only print the value [aliases: only-value]
    -q, --quiet                         Suppress output at a specific level and below
    -S, --stateless                     Ignore local state files, do not read from or write to them
    -v, --verbose                       Display more verbose output
    -V, --version                       Print version information
        --values-width-limit <LIMIT>    Limit the width of the values when using `--format=table` (0 means unlimited)

```

Long help:

```console
$ seaplane metadata list --help
seaplane[EXE]-metadata-list [..]
List one or more metadata key-value pairs

Keys and values will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode allows one to decode them and display the unencoded
values.

USAGE:
    seaplane metadata list <DIR> [OPTIONS]

ARGS:
    <DIR>
            The root directory of the metadata key-value pairs to list

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with a Seaplane account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

    -B, --base64
            The keys/values are already encoded in URL safe Base64

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -D, --decode
            Decode the keys and values before printing them
            
            Binary values will be written directly to standard output (which may do strange
            things to your terminal)

        --decode-safe
            Decode the keys and values in a terminal-friendly way

    -f, --from <KEY>
            Only print metadata key-value pairs after this key (note: if this key has a value it will be included in the results)

        --format <FORMAT>
            Change the output format
            
            [default: table]
            [possible values: table, json]

    -h, --help
            Print help information

    -H, --no-header
            Omit the 'KEY' or 'VALUE' heading when printing with `--format=table`
            
            [aliases: no-heading, no-headers]

        --keys-width-limit <LIMIT>
            Limit the width of the keys when using `--format=table` (0 means unlimited)

        --no-color
            Do not color output (alias for --color=never)

        --no-decode
            Print keys and values without decoding them

        --only-keys
            Only print the key
            
            [aliases: only-key]

        --only-values
            Only print the value
            
            [aliases: only-value]

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

        --values-width-limit <LIMIT>
            Limit the width of the values when using `--format=table` (0 means unlimited)

```
