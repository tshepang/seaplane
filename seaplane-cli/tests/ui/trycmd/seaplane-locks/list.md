Short help:

```console
$ seaplane locks list -h
seaplane-locks-list v0.1.0 (ef6a53e)
Get information around a currently held lock

USAGE:
    seaplane locks list <LOCK_NAME> [OPTIONS]

ARGS:
    <LOCK_NAME>    The name of the lock

OPTIONS:
    -A, --api-key <STRING>           The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
    -B, --base64                     The lockname is already encoded in URL safe Base64
        --color <COLOR>              Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -D, --decode                     Decode the lockname before printing it (WARNING! See --help)
    -E, --display-encoding <KIND>    What format to display the decoded (--decode) lockname (WARNING! See --help) [default: simple] [possible values: simple, utf8, hex]
        --format <FORMAT>            Change the output format [default: table] [possible values: table, json]
    -h, --help                       Print help information
    -H, --no-header                  Omit the heading when printing with `--format=table` [aliases: no-heading, no-headers]
        --no-color                   Do not color output (alias for --color=never)
        --no-decode                  Print lockname without decoding it
    -q, --quiet                      Suppress output at a specific level and below
    -S, --stateless                  Ignore local state files, do not read from or write to them
    -v, --verbose                    Display more verbose output
    -V, --version                    Print version information

```

Long help:

```console
$ seaplane locks list --help
seaplane-locks-list v0.1.0 (ef6a53e)
Get information around a currently held lock.

Locknames will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode allows one to decode them and display the unencoded
values. However since they may contain arbitrary data, it's possible to re-encode them into a
different format for display purposes using --display-encoding

USAGE:
    seaplane locks list <LOCK_NAME> [OPTIONS]

ARGS:
    <LOCK_NAME>
            The name of the lock

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with a Seaplane account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

    -B, --base64
            The lockname is already encoded in URL safe Base64

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -D, --decode
            Decode the lock-id before printing it
            
            WARNING!
            By default the display encoding is `simple` which if the lock-id contains binary data this
            can mess with your terminal! Use `--display-encoding=hex` or `--display-encoding=utf8` if your
            lock-id may contain binary data.

    -E, --display-encoding <KIND>
            What format to display the decoded (--decode) lock-id
            
            WARNING!
            If the value contains binary data using `--display-encoding=simple` can mess with your terminal!
            
            WARNING!
            When using `--display-encoding=simple` or `--display-encoding=utf8` along with `--format=json` the
            result can be invalid JSON if your lock-id contains unescaped characters that are not valid for a
            JSON string. In these cases, unless you're sure your keys and values only contain valid JSON string
            data, you should either use `--display-encoding=hex` or leave the values in their base64 format by
            omitting `--decode` (or use `--no-decode`)
            
            simple => No encoding, just display as is
            utf8   => Lossily encode to UTF-8. Invalid UTF-8 sequences will be converted to U+FFFD REPLACEMENT
                      CHARACTER which looks like this �
            hex    => Raw bytes will be hex encoded and displayed as text
            
            [default: simple]
            [possible values: simple, utf8, hex]

        --format <FORMAT>
            Change the output format
            
            [default: table]
            [possible values: table, json]

    -h, --help
            Print help information

    -H, --no-header
            Omit the heading when printing with `--format=table`
            
            [aliases: no-heading, no-headers]

        --no-color
            Do not color output (alias for --color=never)

        --no-decode
            Print lockname without decoding it

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
