The short help message with `-h`:

```console
$ seaplane formation plan -h
seaplane[EXE]-formation-plan [..]
Create a Seaplane Formation

USAGE:
    seaplane formation plan --include-flight-plan=SPEC... [OPTIONS]

OPTIONS:
    -A, --api-key <STRING>               The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>                  Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --exclude-provider <PROVIDER>    A provider that this Formation's Flights are *NOT* permitted to run on (supports comma separated list, or multiple uses) [aliases: exclude-providers] [possible values: aws, azure, digitalocean, equinix, gcp, all]
        --exclude-region <REGION>        A region in which this Formation's Flights are *NOT* allowed to run in (supports comma separated list, or multiple uses) (See REGION SPEC below) [aliases: exclude-regions] [possible values: xa, xc, xe, xf, xn, xo, xq, xs, xu, all]
    -F, --fetch                          Fetch remote instances prior to creating this plan to check for conflicts (by default only local references are considered) [aliases: sync, synchronize]
        --flight-endpoint <SPEC>         An endpoint that will only be privately exposed on Instances of this Formation Plan to Flights within the same Formation Instance. In the form of 'PROTO:TARGET=FLIGHT:PORT' (supports comma separated list, or multiple uses) [aliases: flight-endpoints]
        --force                          Override any existing Formation with the same NAME
        --grounded                       This Formation Plan should be deployed but NOT set as active (requires a formation configuration) [aliases: no-active]
    -h, --help                           Print help information
    -I, --include-flight-plan <SPEC>     Use local Flight Plan in this Formation in the form of ID|NAME|@path|@-|INLINE-SPEC (supports SEMICOLON (';') separated list, or multiple uses) (See FLIGHT SPEC below) [aliases: include-flight-plans]
        --launch                         This Formation Plan should be deployed and set as active right away (requires a formation configuration) [aliases: active]
    -n, --name <STRING>                  A human readable name for the Formation (must be unique within the tenant) if omitted a pseudo random name will be assigned
        --no-color                       Do not color output (alias for --color=never)
        --provider <PROVIDER>            A provider that this Formation's Flights are permitted to run on (supports comma separated list, or multiple uses) [default: all] [aliases: providers] [possible values: aws, azure, digitalocean, equinix, gcp, all]
        --public-endpoint <SPEC>         An endpoint that will be publicly exposed by instances of this Formation Plan in the form of 'ROUTE=FLIGHT:PORT' (supports comma separated list, or multiple uses) [aliases: public-endpoints]
    -q, --quiet                          Suppress output at a specific level and below
        --region <REGION>                A region in which this Formation's Flights are allowed to run in (supports comma separated list, or multiple uses) (See REGION SPEC below) [default: all] [aliases: regions] [possible values: xa, xc, xe, xf, xn, xo, xq, xs, xu, all]
    -S, --stateless                      Ignore local state files, do not read from or write to them
    -v, --verbose                        Display more verbose output
    -V, --version                        Print version information

FLIGHT SPEC

    The Flight may be specified in one of the following ways

    FLIGHT_SPEC := NAME | ID | @path | @- | INLINE-SPEC
    NAME        := The local Flight Plan name
    ID          := The local hex-encoded ID of the Flight Plan
    @path       := PATH is an existing file with a Flight Plan definition in JSON format
    @-          := STDIN will be read for a Flight Plan definition in JSON format
    INLINE-SPEC := Comma separated LIST of ATTRIBUTE
    ATTRIBUTE   := image=IMAGE [ | name=NAME | minimum=NUM | maximum=NUM | api-permission | architecture=ARCH ]
    NUM         := Positive integer (minimum default is 1 if omitted; maximum default is 'autoscale as needed')
    ARCH        := amd64 | arm64

    NOTE that when using @- only one Flight Plan may be provided via STDIN

REGION SPEC

    The regions are based on ISO 3166 alpha-2 continent codes with a few additions to capture
    regulatory differences along with some more intuitive or common aliases. The currently
    supported mappings are:

    XA => Asia
    XC => PRC => PeoplesRepublicofChina
    XE => EU  => Europe
    XF => Africa
    XN => NAmerica => NorthAmerica
    XO => Oceania
    XQ => Antarctica
    XS => SAmerica => SouthAmerica
    XU => UK => UnitedKingdom

    This list is subject to change or expand.

```

The long help message with `--help`:

```console
$ seaplane formation plan --help
seaplane[EXE]-formation-plan [..]
Make a new local Formation Plan (and optionally launch an instance of it)

Include local Flight Plans by using `--include-flight-plan`. Multiple Flights may be included in a
Formation Plan using a SEMICOLON separated list, or using the argument multiple times.

You can also create a new Flight Plan using the INLINE-SPEC option of `--include-flight-plan`.

Flight Plans created using INLINE-SPEC are automatically included in the Formation Plan.

USAGE:
    seaplane formation plan --include-flight-plan=SPEC... [OPTIONS]

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with a Seaplane account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

        --exclude-provider <PROVIDER>
            A provider that this Formation's Flights are *NOT* permitted to run on
            
            This will override any values given to --provider
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: exclude-providers]
            [possible values: aws, azure, digitalocean, equinix, gcp, all]

        --exclude-region <REGION>
            A region in which this Formation's Flights are *NOT* allowed to run in (See REGION SPEC below)
            
            This will override any values given to --region
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: exclude-regions]
            [possible values: xa, xc, xe, xf, xn, xo, xq, xs, xu, all]

    -F, --fetch
            Fetch remote instances prior to creating this plan to check for conflicts (by default only local references are considered)
            
            [aliases: sync, synchronize]

        --flight-endpoint <SPEC>
            An endpoint that will only be exposed privately on Instances of this Formation Plan (only exposed to Flights within this same Formation Instance)
            
            Flight Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where
            
            PROTO  := [http | https] | tcp | udp
            TARGET := ROUTE | PORT
            ROUTE  := with PROTO http, and HTTP URL route, can be elided
            PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
            FLIGHT := NAME or ID
            PORT   := Network Port (0-65535)
            
            This describes where traffic arriving at this Formation's domain URL from within this Formation's
            private network should be sent.
            
            For example, consider:
            
            $ seaplane formation edit Foo --flight-endpoint udp:1234=baz:4321
            
            Would mean, route all traffic arriving to the 'Foo' Formation's domain URL on UDP/1234 from the
            Formation's private network to the the Formation's Flight named 'baz' on port '4321'. The PROTO of
            the incoming traffic will be used for the PROTO of the outgoing traffic to FLIGHT
            
            Note 'https' can be used interchangeably with 'http' for convenience sake. It does NOT however
            require the traffic actually be HTTPS. Here 'http' (or convenience 'https') simply means "Traffic
            using the HTTP" protocol.
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: flight-endpoints]

        --force
            Override any existing Formation with the same NAME

        --grounded
            This Formation Plan should be deployed but NOT set as active (requires a formation configuration)
            
            [aliases: no-active]

    -h, --help
            Print help information

    -I, --include-flight-plan <SPEC>
            A Flight Plan to include in this Formation in the form of ID|NAME|@path|@-|INLINE-SPEC (See FLIGHT SPEC below)
            
            Multiple items can be passed as a SEMICOLON (';') separated list or by using the argument multiple
            times. Note that when using the INLINE-SPEC it's usually easiest to only place one Flight Plan per
            --include-flight-plan argument
            
            $ seaplane formation plan /
                --include-flight-plan name=flight1,image=nginx:latest /
                --include-flight-plan name=flight2,image=hello:latest
            
            Which would create, and include, two Flight Plans (flight1, and flight2).
            
            [aliases: include-flight-plans]

        --launch
            This Formation Plan should be deployed and set as active right away (requires a formation configuration)
            
            [aliases: active]

    -n, --name <STRING>
            A human readable name for the Formation (must be unique within the tenant)
            
            Rules for a valid name are as follows:
            
              - may only include 0-9, a-z, A-Z, and '-' (hyphen)
              - hyphens ('-') may not be repeated (i.e. '--')
              - no more than three (3) total hyphens
              - the total length must be <= 27
            
            Some of these restrictions may be lifted in the future.

        --no-color
            Do not color output (alias for --color=never)

        --provider <PROVIDER>
            A provider that this Formation's Flights are permitted to run on
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [default: all]
            [aliases: providers]
            [possible values: aws, azure, digitalocean, equinix, gcp, all]

        --public-endpoint <SPEC>
            An endpoint that will publicly exposed on Instances of this Formation Plan
            
            Public Endpoints take the form '{ROUTE}={FLIGHT}:{PORT}'. Where
            
            ROUTE  := An HTTP URL route
            FLIGHT := NAME or ID
            PORT   := Network Port (0-65535)
            
            This describes which Flight and port should serve the HTTP traffic arriving at this Formation's
            domain URL using the specified route.
            
            For example, consider:
            
            $ seaplane formation edit Foo --public-endpoint /foo/bar=baz:1234
            
            Would mean, all HTTP traffic from the public internet hitting the route '/foo/bar' on the 'Foo'
            Formation's domain should be directed to this Formation's Flight named 'baz' on port '1234'
            
            In the future, support for other protocols such as 'tcp:port' or 'udp:port' may be added alongside
            'http' routes.
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: public-endpoints]

    -q, --quiet
            Suppress output at a specific level and below
            
            More uses suppresses higher levels of output
                -q:   Only display WARN messages and above
                -qq:  Only display ERROR messages
                -qqq: Suppress all output

        --region <REGION>
            A region in which this Formation's Flights are allowed to run in (See REGION SPEC below)
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [default: all]
            [aliases: regions]
            [possible values: xa, xc, xe, xf, xn, xo, xq, xs, xu, all]

    -S, --stateless
            Ignore local state files, do not read from or write to them

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

FLIGHT SPEC

    The Flight may be specified in one of the following ways

    FLIGHT_SPEC := NAME | ID | @path | @- | INLINE-SPEC
    NAME        := The local Flight Plan name
    ID          := The local hex-encoded ID of the Flight Plan
    @path       := PATH is an existing file with a Flight Plan definition in JSON format
    @-          := STDIN will be read for a Flight Plan definition in JSON format
    INLINE-SPEC := Comma separated LIST of ATTRIBUTE
    ATTRIBUTE   := image=IMAGE [ | name=NAME | minimum=NUM | maximum=NUM | api-permission | architecture=ARCH ]
    NUM         := Positive integer (minimum default is 1 if omitted; maximum default is 'autoscale as needed')
    ARCH        := amd64 | arm64

    NOTE that when using @- only one Flight Plan may be provided via STDIN

REGION SPEC

    The regions are based on ISO 3166 alpha-2 continent codes with a few additions to capture
    regulatory differences along with some more intuitive or common aliases. The currently
    supported mappings are:

    XA => Asia
    XC => PRC => PeoplesRepublicofChina
    XE => EU  => Europe
    XF => Africa
    XN => NAmerica => NorthAmerica
    XO => Oceania
    XQ => Antarctica
    XS => SAmerica => SouthAmerica
    XU => UK => UnitedKingdom

    This list is subject to change or expand.

```
