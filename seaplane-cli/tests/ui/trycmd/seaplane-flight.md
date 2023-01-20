Be default, `seaplane flight` will display the help text:

```console
$ seaplane flight
? 2
Operate on local Flight Plans which define "Flights" (logical containers), and are then referenced by Formations

Usage: seaplane[EXE] flight [OPTIONS] <COMMAND>

Commands:
  plan    Make a new local Flight Plan that Formations can include and reference [aliases: create, add]
  copy    Copy a local Flight Plan (optionally make changes to the copy) [aliases: clone]
  edit    Edit a local Flight Plan
  delete  Delete a local Flight Plan [aliases: del, remove, rm]
  list    List all local Flight Plans [aliases: ls]
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
