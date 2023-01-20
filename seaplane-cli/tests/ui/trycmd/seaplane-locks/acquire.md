Short help

```console
$ seaplane locks acquire -h
Attempt to acquire the lock for N seconds

Usage: seaplane[EXE] locks acquire [OPTIONS] --ttl <SECS> --client-id <STRING> <LOCK_NAME>

Arguments:
  <LOCK_NAME>  The name of the lock

Options:
      --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
  -T, --ttl <SECS>          The TTL (Time To Live) in seconds, i.e. a positive integer
  -v, --verbose...          Display more verbose output
  -B, --base64              The lockname is already encoded in URL safe Base64
  -q, --quiet...            Suppress output at a specific level and below
      --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
  -L, --client-id <STRING>  Client-chosen identifier stored with the lock for informational purposes
      --no-color            Do not color output (alias for --color=never)
  -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless           Ignore local state files, do not read from or write to them
  -h, --help                Print help (see more with '--help')
  -V, --version             Print version

```

Long help:

```console
$ seaplane locks acquire --help
Attempt to acquire the lock for N seconds

Usage: seaplane[EXE] locks acquire [OPTIONS] --ttl <SECS> --client-id <STRING> <LOCK_NAME>

Arguments:
  <LOCK_NAME>
          The name of the lock

Options:
      --format <FORMAT>
          Change the output format
          
          [default: table]
          [possible values: table, json]

  -T, --ttl <SECS>
          The TTL (Time To Live) in seconds, i.e. a positive integer

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -B, --base64
          The lockname is already encoded in URL safe Base64

  -q, --quiet...
          Suppress output at a specific level and below
          
          More uses suppresses higher levels of output
              -q:   Only display WARN messages and above
              -qq:  Only display ERROR messages
              -qqq: Suppress all output

      --color <COLOR>
          Should the output include color?
          
          [default: auto]
          [possible values: always, ansi, auto, never]

  -L, --client-id <STRING>
          Client-chosen identifier stored with the lock for informational purposes

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
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
