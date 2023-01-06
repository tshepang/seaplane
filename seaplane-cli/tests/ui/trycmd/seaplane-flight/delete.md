
With no additional arguments, an error is displayed about a missing required argument.

```console
$ seaplane flight delete
? 2
error: The following required arguments were not provided:
  <NAME|ID>

Usage: seaplane[EXE] flight delete <NAME|ID>

For more information try '--help'

```

The short help message with `-h`:

```console
$ seaplane flight delete -h
Delete a local Flight Plan

Usage: seaplane[EXE] flight delete [OPTIONS] <NAME|ID>

Arguments:
  <NAME|ID>  The name or ID of the Flight Plan to remove, must be unambiguous

Options:
  -f, --force             Delete this Flight Plan even if referenced by a local Formation Plan, or deletes ALL Flight Plan referenced by the name or ID even if ambiguous
  -v, --verbose...        Display more verbose output
  -a, --all               Delete all matching Flight Plans even when the name or ID is ambiguous
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help information (use `--help` for more detail)
  -V, --version           Print version information

```

The long help message with `--help`:

```console
$ seaplane flight delete --help
Delete a local Flight Plan

Usage: seaplane[EXE] flight delete [OPTIONS] <NAME|ID>

Arguments:
  <NAME|ID>
          The name or ID of the Flight Plan to remove, must be unambiguous

Options:
  -f, --force
          Delete this Flight Plan even if referenced by a local Formation Plan, or deletes ALL Flight Plan referenced by the name or ID even if ambiguous

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -a, --all
          Delete all matching Flight Plans even when the name or ID is ambiguous

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
