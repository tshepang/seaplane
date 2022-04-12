```console
$ seaplane formation fetch-remote -h
Fetch remote Formation definitions
seaplane-formation-fetch-remote [..]

USAGE:
    seaplane formation fetch-remote
    seaplane formation fetch-remote [NAME|ID]

ARGS:
    <NAME|ID>    The NAME or ID of the formation to fetch, omit to fetch all Formations

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

```console
$ seaplane formation fetch-remote --help
Fetch remote Formation definitions
seaplane-formation-fetch-remote [..]

USAGE:
    seaplane formation fetch-remote
    seaplane formation fetch-remote [NAME|ID]

ARGS:
    <NAME|ID>
            The NAME or ID of the formation to fetch, omit to fetch all Formations

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
