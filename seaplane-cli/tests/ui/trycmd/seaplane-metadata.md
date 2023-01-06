With no args:

```console
$ seaplane metadata
? 2
Operate on metadata key-value pairs using the Global Data Coordination API

Usage: seaplane[EXE] metadata [OPTIONS] <COMMAND>

Commands:
  get     Retrieve a metadata key-value pair [aliases: show]
  set     Set a metadata key-value pair [aliases: put]
  delete  Delete one or more metadata key-value pairs [aliases: del, remove, rm]
  list    List one or more metadata key-value pairs [aliases: ls]
  help    Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>   Change the output format [default: table] [possible values: table, json]
  -v, --verbose...        Display more verbose output
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help information (use `--help` for more detail)
  -V, --version           Print version information

```

The short help:

```console
$ seaplane metadata -h
Operate on metadata key-value pairs using the Global Data Coordination API

Usage: seaplane[EXE] metadata [OPTIONS] <COMMAND>

Commands:
  get     Retrieve a metadata key-value pair [aliases: show]
  set     Set a metadata key-value pair [aliases: put]
  delete  Delete one or more metadata key-value pairs [aliases: del, remove, rm]
  list    List one or more metadata key-value pairs [aliases: ls]
  help    Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>   Change the output format [default: table] [possible values: table, json]
  -v, --verbose...        Display more verbose output
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help information (use `--help` for more detail)
  -V, --version           Print version information

```

The long help:

```console
$ seaplane metadata --help
Operate on metadata key-value pairs using the Global Data Coordination API

Usage: seaplane[EXE] metadata [OPTIONS] <COMMAND>

Commands:
  get
          Retrieve a metadata key-value pair [aliases: show]
  set
          Set a metadata key-value pair [aliases: put]
  delete
          Delete one or more metadata key-value pairs [aliases: del, remove, rm]
  list
          List one or more metadata key-value pairs [aliases: ls]
  help
          Print this message or the help of the given subcommand(s)

Options:
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
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information

```
