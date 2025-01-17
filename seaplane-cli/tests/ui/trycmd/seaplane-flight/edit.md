
With no additional arguments, an error is displayed about a missing required argument.

```console
$ seaplane flight edit
? 2
error: the following required arguments were not provided:
  <NAME|ID>

Usage: seaplane[EXE] flight edit <NAME|ID>

For more information, try '--help'.

```

The short help message with `-h`:

```console
$ seaplane flight edit -h
Edit a local Flight Plan

Usage: seaplane[EXE] flight edit [OPTIONS] <NAME|ID>

Arguments:
  <NAME|ID>  The source name or ID of the Flight Plan to edit

Options:
  -v, --verbose...           Display more verbose output
  -x, --exact                The given name or ID must be an exact match
      --image <SPEC>         The container image registry reference that this Flight will use (See IMAGE SPEC below) [aliases: img]
  -q, --quiet...             Suppress output at a specific level and below
      --color <COLOR>        Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
  -n, --name <STRING>        A human readable name for the Flight (must be unique within any Formation it is a part of) if omitted a pseudo random name will be assigned
      --minimum <NUM>        The minimum number of container instances that should ever be running [default: 1] [aliases: min]
      --no-color             Do not color output (alias for --color=never)
  -A, --api-key <STRING>     The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
      --maximum <NUM>        The maximum number of container instances that should ever be running (default: autoscale as needed) [aliases: max]
      --architecture <ARCH>  The architectures this flight is capable of running on. No value means it will be auto detected from the image definition (supports comma separated list, or multiple uses) [aliases: arch, arches, architectures] [possible values: amd64, arm64]
  -S, --stateless            Ignore local state files, do not read from or write to them
      --no-maximum           There is no maximum number of instances [aliases: no-max]
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version

IMAGE SPEC

    NOTE that a default registry of `registry.cplane.cloud` is used.

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

    registry.cplane.cloud/busybox@sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa
    registry.cplane.cloud/seaplane/busybox:latest

```

The long help message with `--help`:

```console
$ seaplane flight edit --help
Edit a local Flight Plan

Usage: seaplane[EXE] flight edit [OPTIONS] <NAME|ID>

Arguments:
  <NAME|ID>
          The source name or ID of the Flight Plan to edit

Options:
  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -x, --exact
          The given name or ID must be an exact match

      --image <SPEC>
          The container image registry reference that this Flight will use (See IMAGE SPEC below)
          
          NOTE at this time the if the registry is omitted, such as `nginxdemos/hello:latest` a default
          registry of `registry.cplane.cloud` will be used. This may change in the future, so it is
          recommended to always specify a full image reference path.
          
          [aliases: img]

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

  -n, --name <STRING>
          A human readable name for the Flight (must be unique within any Formation it
          
          Rules for a valid name are as follows:
          
            - may only include 0-9, a-z, A-Z, and '-' (hyphen)
            - hyphens ('-') may not be repeated (i.e. '--')
            - no more than three (3) total hyphens
            - the total length must be <= 27
          
          Some of these restrictions may be lifted in the future.

      --minimum <NUM>
          The minimum number of container instances that should ever be running
          
          [default: 1]
          [aliases: min]

      --no-color
          Do not color output (alias for --color=never)

  -A, --api-key <STRING>
          The API key associated with a Seaplane account used to access Seaplane API endpoints
          
          The value provided here will override any provided in any configuration files.
          A CLI provided value also overrides any environment variables.
          One can use a special value of '-' to signal the value should be read from STDIN.
          
          [env: SEAPLANE_API_KEY]

      --maximum <NUM>
          The maximum number of container instances that should ever be running (default: autoscale as needed)
          
          [aliases: max]

      --architecture <ARCH>
          The architectures this flight is capable of running on. No value means it will be auto detected from the image definition
          
          Multiple items can be passed as a comma separated list, or by using the argument
          multiple times.
          
          [aliases: arch, arches, architectures]
          [possible values: amd64, arm64]

  -S, --stateless
          Ignore local state files, do not read from or write to them

      --no-maximum
          There is no maximum number of instances
          
          [aliases: no-max]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

IMAGE SPEC

    NOTE that a default registry of `registry.cplane.cloud` is used.

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

    registry.cplane.cloud/busybox@sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa
    registry.cplane.cloud/seaplane/busybox:latest

```
