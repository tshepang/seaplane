Short help:

```console
$ seaplane key-value list -h
seaplane-key-value-list [PKGVER]
List one or more key-value pairs

USAGE:
    seaplane key-value list <DIR> [OPTIONS]

ARGS:
    <DIR>    The root directory of the key-value pairs to list

OPTIONS:
    -a, --after                      Only print key-value pairs after this key (note: this key and it's value are NOT included in the results)
    -A, --api-key <STRING>           The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
    -B, --base64                     The keys/values are already encoded in URL safe Base64
        --color <COLOR>              Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -D, --decode                     Decode the keys and values before printing them (WARNING! See --help)
    -E, --display-encoding <KIND>    What format to display the decoded (--decode) keys/values (WARNING! See --help) [default: simple] [possible values: simple, utf8, hex]
    -h, --help                       Print help information
    -H, --no-header                  Omit the 'KEY' or 'VALUE' heading when printing with `--format=table` [aliases: no-heading]
        --no-color                   Do not color output (alias for --color=never)
        --no-decode                  Print keys and values without decoding them
        --only-key                   Only print the key [aliases: only-keys]
        --only-value                 Only print the value [aliases: only-values]
    -q, --quiet                      Suppress output at a specific level and below
    -v, --verbose                    Display more verbose output
    -V, --version                    Print version information

```

Long help:

```console
$ seaplane key-value list --help
seaplane-key-value-list [PKGVER]
List one or more key-value pairs

USAGE:
    seaplane key-value list <DIR> [OPTIONS]

ARGS:
    <DIR>
            The root directory of the key-value pairs to list

OPTIONS:
    -a, --after
            Only print key-value pairs after this key (note: this key and it's value are NOT included in the results)

    -A, --api-key <STRING>
            The API key associated with your account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

    -B, --base64
            The keys/values are already encoded in URL safe Base64

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -D, --decode
            Decode the keys and values before printing them
            
            WARNING!
            By default the display encoding is `simple` which if the keys or values contain binary data this
            can mess with your terminal! Use `--display-encoding=hex` or `--display-encoding=utf8` if your
            values may contain binary data.

    -E, --display-encoding <KIND>
            What format to display the decoded (--decode) keys/values
            
            WARNING!
            If the value contains binary data using `--display-encoding=simple` can mess with your terminal!
            
            WARNING!
            When using `--display-encoding=simple` or `--display-encoding=utf8` along with `--format=json` the
            result can be invalid JSON if your keys or values contain unescaped characters that are not valid
            for a JSON string. In these cases, unless you're sure your keys and values only contain valid JSON
            string data, you should either use `--display-encoding=hex` or leave the values in their base64
            format by omitting `--decode` (or use `--no-decode`)
            
            simple => No encoding, just display as is
            utf8   => Lossily encode to UTF-8. Invalid UTF-8 sequences will be converted to U+FFFD REPLACEMENT
                      CHARACTER which looks like this �
            hex    => Raw bytes will hex encoded and displayed as text
            
            [default: simple]
            [possible values: simple, utf8, hex]

    -h, --help
            Print help information

    -H, --no-header
            Omit the 'KEY' or 'VALUE' heading when printing with `--format=table`
            
            [aliases: no-heading]

        --no-color
            Do not color output (alias for --color=never)

        --no-decode
            Print keys and values without decoding them

        --only-key
            Only print the key
            
            [aliases: only-keys]

        --only-value
            Only print the value
            
            [aliases: only-values]

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
