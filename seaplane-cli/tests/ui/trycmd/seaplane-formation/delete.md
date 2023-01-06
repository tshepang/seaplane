```console
$ seaplane formation delete -h
Deletes local Formation Plans and/or remote Formation Instances

Usage: 
    seaplane formation delete [OPTIONS] <NAME|ID>
    seaplane formation delete [OPTIONS] <NAME|ID> --no-remote

Arguments:
  <NAME|ID>  The name or ID of the Formation to remove, must be unambiguous

Options:
  -r, --recursive         Recursively delete all local definitions associated with this Formation
  -v, --verbose...        Display more verbose output
  -f, --force             Delete this Formation even if there are remote instances In Flight (active), which will effectively stop all remote instances of this Formation
  -q, --quiet...          Suppress output at a specific level and below
  -a, --all               Delete all matching Formations even when the name or ID is ambiguous or a partial match
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --local             Delete local Formation Definitions (this is set by the default, use --no-local to skip)
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
      --no-local          DO NOT delete local Formation Definitions
      --remote            Delete remote Formation Instances (this is set by default, use --no-remote to skip)
  -S, --stateless         Ignore local state files, do not read from or write to them
      --no-remote         DO NOT delete remote Formation Instances (this is set by the default, use --remote to remove them)
  -F, --fetch             Fetch remote Formation Instances and synchronize local Plan definitions prior to attempting to delete [aliases: sync, synchronize]
  -h, --help              Print help information (use `--help` for more detail)
  -V, --version           Print version information

```

```console
$ seaplane formation delete --help
Deletes local Formation Plans and/or remote Formation Instances

Usage: 
    seaplane formation delete [OPTIONS] <NAME|ID>
    seaplane formation delete [OPTIONS] <NAME|ID> --no-remote

Arguments:
  <NAME|ID>
          The name or ID of the Formation to remove, must be unambiguous

Options:
  -r, --recursive
          Recursively delete all local definitions associated with this Formation

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -f, --force
          Delete this Formation even if there are remote instances In Flight (active), which will effectively stop all remote instances of this Formation

  -q, --quiet...
          Suppress output at a specific level and below
          
          More uses suppresses higher levels of output
              -q:   Only display WARN messages and above
              -qq:  Only display ERROR messages
              -qqq: Suppress all output

  -a, --all
          Delete all matching Formations even when the name or ID is ambiguous or a partial match

      --color <COLOR>
          Should the output include color?
          
          [default: auto]
          [possible values: always, ansi, auto, never]

      --local
          Delete local Formation Definitions (this is set by the default, use --no-local to skip)

      --no-color
          Do not color output (alias for --color=never)

  -A, --api-key <STRING>
          The API key associated with a Seaplane account used to access Seaplane API endpoints
          
          The value provided here will override any provided in any configuration files.
          A CLI provided value also overrides any environment variables.
          One can use a special value of '-' to signal the value should be read from STDIN.
          
          [env: SEAPLANE_API_KEY]

      --no-local
          DO NOT delete local Formation Definitions

      --remote
          Delete remote Formation Instances (this is set by default, use --no-remote to skip)

  -S, --stateless
          Ignore local state files, do not read from or write to them

      --no-remote
          DO NOT delete remote Formation Instances (this is set by the default, use --remote to remove them)

  -F, --fetch
          Fetch remote Formation Instances and synchronize local Plan definitions prior to attempting to delete
          
          [aliases: sync, synchronize]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information

```
