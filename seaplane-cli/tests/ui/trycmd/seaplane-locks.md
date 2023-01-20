With no args:

```console
$ seaplane locks
? 2
Operate on the Locks API

Usage: seaplane[EXE] locks [OPTIONS] <COMMAND>

Commands:
  list     Get information around currently held locks [aliases: ls]
  acquire  Attempt to acquire the lock for N seconds [aliases: acq]
  release  Attempt to release a lock [aliases: rl]
  renew    Attempt to renew the lock for N seconds
  help     Print this message or the help of the given subcommand(s)

Options:
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

The short help:

```console
$ seaplane locks -h
Operate on the Locks API

Usage: seaplane[EXE] locks [OPTIONS] <COMMAND>

Commands:
  list     Get information around currently held locks [aliases: ls]
  acquire  Attempt to acquire the lock for N seconds [aliases: acq]
  release  Attempt to release a lock [aliases: rl]
  renew    Attempt to renew the lock for N seconds
  help     Print this message or the help of the given subcommand(s)

Options:
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
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
