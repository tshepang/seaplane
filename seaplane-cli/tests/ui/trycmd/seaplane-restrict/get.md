Short help:

```console
$ seaplane restrict get -h
Retrieve information about a directory restriction

Usage: seaplane[EXE] restrict get [OPTIONS] <API> <DIRECTORY>

Arguments:
  <API>        The API of the restricted directory
  <DIRECTORY>  The restricted directory

Options:
  -B, --base64            The directory is already encoded in URL safe Base64
  -v, --verbose...        Display more verbose output
      --format <FORMAT>   Change the output format [default: table] [possible values: table, json]
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
  -D, --decode            Decode the directories before printing them
      --no-color          Do not color output (alias for --color=never)
      --no-decode         Print directories without decoding them
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
      --no-header         Omit the header when printing with `--format=table` [aliases: no-heading, no-headers]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version

```

Long help:

```console
$ seaplane restrict get --help
Get information about restrictions on a directory

Directory will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Use --decode to output the decoded values instead.

Usage: seaplane[EXE] restrict get [OPTIONS] <API> <DIRECTORY>

Arguments:
  <API>
          The API of the restricted directory

  <DIRECTORY>
          The restricted directory

Options:
  -B, --base64
          The directory is already encoded in URL safe Base64

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

      --format <FORMAT>
          Change the output format
          
          [default: table]
          [possible values: table, json]

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

  -D, --decode
          Decode the directories before printing them
          
          Binary values will be written directly to standard output (which may do strange
          things to your terminal)

      --no-color
          Do not color output (alias for --color=never)

      --no-decode
          Print directories without decoding them

  -A, --api-key <STRING>
          The API key associated with a Seaplane account used to access Seaplane API endpoints
          
          The value provided here will override any provided in any configuration files.
          A CLI provided value also overrides any environment variables.
          One can use a special value of '-' to signal the value should be read from STDIN.
          
          [env: SEAPLANE_API_KEY]

      --no-header
          Omit the header when printing with `--format=table`
          
          [aliases: no-heading, no-headers]

  -S, --stateless
          Ignore local state files, do not read from or write to them

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
