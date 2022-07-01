Short help:

```console
$ seaplane locks renew -h
seaplane-locks-renew v0.1.0 (ef6a53e)
Attempt to renew the lock for TTL seconds

USAGE:
    seaplane locks renew <LOCK_NAME> --lock-id LOCK_ID --ttl TTL [OPTIONS]

ARGS:
    <LOCK_NAME>    The name of the lock

OPTIONS:
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
    -B, --base64              The lockname is already encoded in URL safe Base64
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
    -h, --help                Print help information
    -L, --lock-id <LOCK_ID>    A valid lock-id can be obtained from a successful acquisition, or listing of the locks
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -S, --stateless           Ignore local state files, do not read from or write to them
    -T, --ttl <TTL>           The TTL (Time To Live) in seconds, i.e. a positive integer
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

Long Help:

```console
$ seaplane locks renew --help
seaplane-locks-renew v0.1.0 (ef6a53e)
Attempt to renew the lock for TTL seconds

USAGE:
    seaplane locks renew <LOCK_NAME> --lock-id LOCK_ID --ttl TTL [OPTIONS]

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

        --format <FORMAT>
            Change the output format

            [default: table]
            [possible values: table, json]

    -h, --help
            Print help information

    -L, --lock-id <LOCK_ID>
            A valid lock-id can be obtained from a successful acquisition, or listing of the locks

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

    -T, --ttl <TTL>
            The TTL (Time To Live) in seconds, i.e. a positive integer

    -v, --verbose
            Display more verbose output

            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

```
