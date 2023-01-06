Short help:

```console
$ seaplane locks renew -h
Attempt to renew the lock for N seconds

Usage: seaplane[EXE] locks renew [OPTIONS] --lock-id <STRING> --ttl <SECS> <LOCK_NAME>

Arguments:
  <LOCK_NAME>  The name of the lock

Options:
      --format <FORMAT>   Change the output format [default: table] [possible values: table, json]
  -L, --lock-id <STRING>  A valid lock-id can be obtained from a successful acquisition, or listing of the locks
  -v, --verbose...        Display more verbose output
  -q, --quiet...          Suppress output at a specific level and below
  -T, --ttl <SECS>        The TTL (Time To Live) in seconds, i.e. a positive integer
  -B, --base64            The lockname is already encoded in URL safe Base64
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help information (use `--help` for more detail)
  -V, --version           Print version information

```

Long Help:

```console
$ seaplane locks renew --help
Attempt to renew the lock for N seconds

Usage: seaplane[EXE] locks renew [OPTIONS] --lock-id <STRING> --ttl <SECS> <LOCK_NAME>

Arguments:
  <LOCK_NAME>
          The name of the lock

Options:
      --format <FORMAT>
          Change the output format
          
          [default: table]
          [possible values: table, json]

  -L, --lock-id <STRING>
          A valid lock-id can be obtained from a successful acquisition, or listing of the locks

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -q, --quiet...
          Suppress output at a specific level and below
          
          More uses suppresses higher levels of output
              -q:   Only display WARN messages and above
              -qq:  Only display ERROR messages
              -qqq: Suppress all output

  -T, --ttl <SECS>
          The TTL (Time To Live) in seconds, i.e. a positive integer

  -B, --base64
          The lockname is already encoded in URL safe Base64

      --color <COLOR>
          Should the output include color?
          
          [default: auto]
          [possible values: always, ansi, auto, never]

      --no-color
          Do not color output (alias for --color=never)

  -A, --api-key <STRING>
          The API key associated with a Seaplane account used to access Seaplane API endpoints
          
          The value provided here will override any provided in any configuration files.
          A CLI provided value also overrides any environment variables.
          One can use a special value of '-' to signal the value should be read from STDIN.
          
          [env: SEAPLANE_API_KEY]

  -S, --stateless
          Ignore local state files, do not read from or write to them

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information

```
