Short help message with `-h`:

```console
$ seaplane init -h
seaplane-init [PKGVER]
Create the Seaplane directory structure at the appropriate locations

USAGE:
    seaplane init [OPTIONS]

OPTIONS:
        --color <COLOR>    Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --force            Force create the files and directories (DANGER: will overwrite existing files)
    -h, --help             Print help information
        --no-color         Do not color output (alias for --color=never)
    -q, --quiet            Suppress output at a specific level and below
    -v, --verbose          Display more verbose output
    -V, --version          Print version information

```

The long help message with `--help`:

```console
$ seaplane init --help
seaplane-init [PKGVER]
Create the Seaplane directory structure at the appropriate locations

USAGE:
    seaplane init [OPTIONS]

OPTIONS:
        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

        --force
            Force create the files and directories (DANGER: will overwrite existing files)

    -h, --help
            Print help information

        --no-color
            Do not color output (alias for --color=never)

    -q, --quiet
            Suppress output at a specific level and below
            
            More uses suppresses higher levels of output
                -q:   Only display WARN messages and above
                -qq:  Only display ERROR messages
                -qqq: Suppress all output

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

```
