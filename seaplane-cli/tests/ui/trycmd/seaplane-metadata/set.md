Short help:

```console
$ seaplane metadata set -h
Set a metadata key-value pair

Usage: seaplane[EXE] metadata set [OPTIONS] <KEY> <VALUE>

Arguments:
  <KEY>    The key to set
  <VALUE>  The value (@path will load the value from a path and @- will load the value from STDIN)

Options:
  -B, --base64            The keys/values are already encoded in URL safe Base64
      --format <FORMAT>   Change the output format [default: table] [possible values: table, json]
  -v, --verbose...        Display more verbose output
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version

```

Long Help: 

```console
$ seaplane metadata set --help
Set a metadata key-value pair

Usage: seaplane[EXE] metadata set [OPTIONS] <KEY> <VALUE>

Arguments:
  <KEY>
          The key to set

  <VALUE>
          The value (@path will load the value from a path and @- will load the value from STDIN)

Options:
  -B, --base64
          The keys/values are already encoded in URL safe Base64

      --format <FORMAT>
          Change the output format
          
          [default: table]
          [possible values: table, json]

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
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
