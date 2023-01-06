Short help:

```console
$ seaplane locks list -h
Get information around currently held locks

Usage: seaplane[EXE] locks list [OPTIONS] [LOCK_NAME]

Arguments:
  [LOCK_NAME]  The name of a lock. If omitted, all locks are shown. Append a trailing slash to list directory contents

Options:
  -B, --base64            The lockname is already encoded in URL safe Base64
      --format <FORMAT>   Change the output format [default: table] [possible values: table, json]
  -v, --verbose...        Display more verbose output
  -D, --decode            Decode the lockname before printing it (WARNING! See --help)
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --no-decode         Print lockname without decoding it
      --no-color          Do not color output (alias for --color=never)
      --no-header         Omit the heading when printing with `--format=table` [aliases: no-heading, no-headers]
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help information (use `--help` for more detail)
  -V, --version           Print version information

```

Long help:

```console
$ seaplane locks list --help
Get information around currently held locks.

There are 3 ways to list locks with this command:
- Omit the LOCK_NAME argument to list all locks
- Use a single lock name as the argument, without a trailing slash, this will list only that single lock
- Use a lock name followed by a trailing slash to list all locks under that directory

Locknames will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode to output the decoded values instead.

Usage: seaplane[EXE] locks list [OPTIONS] [LOCK_NAME]

Arguments:
  [LOCK_NAME]
          The name of a lock. If omitted, all locks are shown. Append a trailing slash to list directory contents

Options:
  -B, --base64
          The lockname is already encoded in URL safe Base64

      --format <FORMAT>
          Change the output format
          
          [default: table]
          [possible values: table, json]

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -D, --decode
          Decode the lock name before printing it
          
          Binary values will be written directly to standard output (which may do strange
          things to your terminal)

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

      --no-decode
          Print lockname without decoding it

      --no-color
          Do not color output (alias for --color=never)

      --no-header
          Omit the heading when printing with `--format=table`
          
          [aliases: no-heading, no-headers]

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
