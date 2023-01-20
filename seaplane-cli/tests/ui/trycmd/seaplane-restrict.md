With no args:

```console
$ seaplane restrict
? 2
Restrict the placement of data for Global Data Coordination API

Usage: seaplane[EXE] restrict [OPTIONS] <COMMAND>

Commands:
  get     Retrieve information about a directory restriction [aliases: show]
  list    List restrictions in an API, or across all APIs [aliases: ls]
  set     Set a restriction [aliases: put]
  delete  Delete a restriction on directory [aliases: del, remove, rm]
  help    Print this message or the help of the given subcommand(s)

Options:
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
$ seaplane restrict -h
Restrict the placement of data for Global Data Coordination API

Usage: seaplane[EXE] restrict [OPTIONS] <COMMAND>

Commands:
  get     Retrieve information about a directory restriction [aliases: show]
  list    List restrictions in an API, or across all APIs [aliases: ls]
  set     Set a restriction [aliases: put]
  delete  Delete a restriction on directory [aliases: del, remove, rm]
  help    Print this message or the help of the given subcommand(s)

Options:
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
$ seaplane restrict --help
Restrict the placement of data for Global Data Coordination API

Usage: seaplane[EXE] restrict [OPTIONS] <COMMAND>

Commands:
  get
          Retrieve information about a directory restriction [aliases: show]
  list
          List restrictions in an API, or across all APIs [aliases: ls]
  set
          Set a restriction [aliases: put]
  delete
          Delete a restriction on directory [aliases: del, remove, rm]
  help
          Print this message or the help of the given subcommand(s)

Options:
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
