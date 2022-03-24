Short help

```console
$ seaplane key-value delete -h
seaplane-key-value-delete [PKGVER]
Delete one or more key-value pairs

USAGE:
    seaplane key-value delete <KEY>... [OPTIONS]

ARGS:
    <KEY>    The key(s) of the key-value pair use

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
    -B, --base64              The keys/values are already encoded in URL safe Base64
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

Long help:

```console
$ seaplane key-value delete --help
seaplane-key-value-delete [PKGVER]
Delete one or more key-value pairs

USAGE:
    seaplane key-value delete <KEY>... [OPTIONS]

ARGS:
    <KEY>
            The key(s) of the key-value pair use

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with your account used to access Seaplane API endpoints
            
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

```
