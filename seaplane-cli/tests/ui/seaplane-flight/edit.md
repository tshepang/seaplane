
With no additional arguments, an error is displayed about a missing required argument.

```console
$ seaplane flight edit
? 2
error: The following required arguments were not provided:
    <NAME|ID>

USAGE:
    seaplane flight edit <NAME|ID> [OPTIONS]

For more information try --help

```

The short help message with `-h`:

```console
$ seaplane flight edit -h
seaplane-flight-edit [PKGVER]
Edit a Flight definition

USAGE:
    seaplane flight edit <NAME|ID> [OPTIONS]

ARGS:
    <NAME|ID>    The source name or ID of the Flight to copy

OPTIONS:
    -A, --api-key <STRING>       The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --api-permission         This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime [aliases: api-permissions]
        --architecture <ARCH>    The architectures this flight is capable of running on. No value means it will be auto detected from the image definition [aliases: arch, arches, architectures] [possible values: amd64, arm64]
        --color <COLOR>          Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                   Print help information
        --image <SPEC>           The container image registry reference that this Flight will use (See IMAGE SPEC below) [aliases: img]
        --maximum <NUM>          The maximum number of container instances that should ever be running (default: infinite) [aliases: max]
        --minimum <NUM>          The minimum number of container instances that should ever be running [default: 1] [aliases: min]
    -n, --name <STRING>          A human readable name for the Flight (must be unique within any Formation it is a part of) if omitted a pseudo random name will be assigned
        --no-api-permission      This Flight should NOT be allowed to hit Seaplane API endpoints and will NOT be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime [aliases: no-api-permissions]
        --no-color               Do not color output (alias for --color=never)
        --no-maximum             There is no maximum number of instances [aliases: no-max]
    -q, --quiet                  Suppress output at a specific level and below
    -v, --verbose                Display more verbose output
    -V, --version                Print version information
    -x, --exact                  The given SOURCE must be an exact match

IMAGE SPEC

    NOTE that at this point the only domain supported is `registry.seaplanet.io`. Other registries
    may be added in the future.

    Valid images can be defined using the grammar

    reference                       := name [ ":" tag ] [ "@" digest ]
    name                            := [domain '/'] path-component ['/' path-component]*
    domain                          := domain-component ['.' domain-component]* [':' port-number]
    domain-component                := /([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])/
    port-number                     := /[0-9]+/
    path-component                  := alpha-numeric [separator alpha-numeric]*
    alpha-numeric                   := /[a-z0-9]+/
    separator                       := /[_.]|__|[-]*/

    tag                             := [..]

    digest                          := digest-algorithm ":" digest-hex
    digest-algorithm                := digest-algorithm-component [ digest-algorithm-separator digest-algorithm-component ]*
    digest-algorithm-separator      := /[+.-_]/
    digest-algorithm-component      := /[A-Za-z][A-Za-z0-9]*/
    digest-hex                      := /[0-9a-fA-F]{32,}/ ; At least 128 bit digest value

    identifier                      := /[a-f0-9]{64}/
    short-identifier                := /[a-f0-9]{6,64}/

    EXAMPLES

    registry.seaplanet.io/library/busybox@sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa
    registry.seaplanet.io/seaplane/busybox:latest

```

The long help message with `--help`:

```console
$ seaplane flight edit --help
seaplane-flight-edit [PKGVER]
Edit a Flight definition

USAGE:
    seaplane flight edit <NAME|ID> [OPTIONS]

ARGS:
    <NAME|ID>
            The source name or ID of the Flight to copy

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with your account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

        --api-permission
            This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime
            
            [aliases: api-permissions]

        --architecture <ARCH>
            The architectures this flight is capable of running on. No value means it will be auto detected from the image definition
            
            [aliases: arch, arches, architectures]
            [possible values: amd64, arm64]

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -h, --help
            Print help information

        --image <SPEC>
            The container image registry reference that this Flight will use (See IMAGE SPEC below)
            
            All image references using the 'registry.seaplanet.io' registry may omit the domain portions of the
            image reference as it is implied. For example, 'registry.seaplanet.io/USER/myimage:latest' can be
            supplied simply as 'USER/myimage:latest'
            
            NOTE at this time the only registry supported is registry.seaplanet.io. In the future when other
            registries are supported, you must specify the full registry domain and path if using those
            alternate registries in order to properly reference your image.
            
            [aliases: img]

        --maximum <NUM>
            The maximum number of container instances that should ever be running (default: infinite)
            
            [aliases: max]

        --minimum <NUM>
            The minimum number of container instances that should ever be running
            
            [default: 1]
            [aliases: min]

    -n, --name <STRING>
            A human readable name for the Flight (must be unique within any Formation it
            
            Rules for a valid name are as follows:
            
              - may only include 0-9, a-z, A-Z, and '-' (hyphen)
              - hyphens ('-') may not be repeated (i.e. '--')
              - no more than three (3) total hyphens
              - the total length must be <= 27
            
            Some of these restrictions may be lifted in the future.

        --no-api-permission
            This Flight should NOT be allowed to hit Seaplane API endpoints and will NOT be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime
            
            [aliases: no-api-permissions]

        --no-color
            Do not color output (alias for --color=never)

        --no-maximum
            There is no maximum number of instances
            
            [aliases: no-max]

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

    -x, --exact
            The given SOURCE must be an exact match

IMAGE SPEC

    NOTE that at this point the only domain supported is `registry.seaplanet.io`. Other registries
    may be added in the future.

    Valid images can be defined using the grammar

    reference                       := name [ ":" tag ] [ "@" digest ]
    name                            := [domain '/'] path-component ['/' path-component]*
    domain                          := domain-component ['.' domain-component]* [':' port-number]
    domain-component                := /([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])/
    port-number                     := /[0-9]+/
    path-component                  := alpha-numeric [separator alpha-numeric]*
    alpha-numeric                   := /[a-z0-9]+/
    separator                       := /[_.]|__|[-]*/

    tag                             := [..]

    digest                          := digest-algorithm ":" digest-hex
    digest-algorithm                := digest-algorithm-component [ digest-algorithm-separator digest-algorithm-component ]*
    digest-algorithm-separator      := /[+.-_]/
    digest-algorithm-component      := /[A-Za-z][A-Za-z0-9]*/
    digest-hex                      := /[0-9a-fA-F]{32,}/ ; At least 128 bit digest value

    identifier                      := /[a-f0-9]{64}/
    short-identifier                := /[a-f0-9]{6,64}/

    EXAMPLES

    registry.seaplanet.io/library/busybox@sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa
    registry.seaplanet.io/seaplane/busybox:latest

```
