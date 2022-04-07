The short help message with `-h`:

```console
$ seaplane formation create -h
seaplane-formation-create [PKGVER]
Create a Seaplane Formation

USAGE:
    seaplane formation create [OPTIONS]
    seaplane formation create --flight=SPEC... [FORMATION CFG OPTIONS]
    seaplane formation create --flight-image=SPEC [INLINE FLIGHT OPTIONS] [FORMATION CFG OPTIONS]

OPTIONS:
    -A, --api-key <STRING>               The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --affinity <NAME|ID>             A Formation that this Formation has an affinity for (supports comma separated list, or multiple uses) [aliases: affinities]
        --color <COLOR>                  Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
        --connection <NAME|ID>           A Formations that this Formation is connected to (supports comma separated list, or multiple uses) [aliases: connections]
        --exclude-provider <PROVIDER>    A provider that this Formation's Flights are *NOT* permitted to run on (supports comma separated list, or multiple uses) [aliases: exclude-providers] [possible values: aws, azure, digitalocean, equinix, gcp, all]
        --exclude-region <REGION>        A region in which this Formation's Flights are *NOT* allowed to run in (supports comma separated list, or multiple uses) (See REGION SPEC below) [aliases: exclude-regions] [possible values: xa, asia, xc, prc, peoplesrepublicofchina, xe, europe, eu, xf, africa, xn, northamerica, namerica, xo, oceania, xq, antarctica, xs, samerica, southamerica, xu, uk, unitedkingdom, all]
    -F, --fetch                          Fetch remote definitions prior to creating to check for conflicts (by default only local state is considered)
        --flight <SPEC>                  A Flight to add to this formation in the form of ID|NAME|@path|@- (supports comma separated list, or multiple uses) (See FLIGHT SPEC below) [aliases: flights]
        --flight-endpoint <SPEC>         An endpoint exposed only to Flights within this Formation. In the form of 'PROTO:TARGET=FLIGHT:PORT' (supports comma separated list, or multiple uses) [aliases: flight-endpoints]
        --force                          Override any existing Formation with the same NAME
        --formation-endpoint <SPEC>      An endpoints exposed only to other Formations privately. In the form of 'PROTO:TARGET=FLIGHT:PORT' (supports comma separated list, or multiple uses) [aliases: formation-endpoints]
        --grounded                       This Formation configuration should be deployed but NOT set as active (requires a formation configuration) [aliases: no-active]
    -h, --help                           Print help information
        --launch                         This Formation configuration should be deployed and set as active right away (requires a formation configuration) [aliases: active]
    -n, --name <STRING>                  A human readable name for the Formation (must be unique within the tenant) if omitted a pseudo random name will be assigned
        --no-color                       Do not color output (alias for --color=never)
        --provider <PROVIDER>            A provider that this Formation's Flights are permitted to run on (supports comma separated list, or multiple uses) [default: all] [aliases: providers] [possible values: aws, azure, digitalocean, equinix, gcp, all]
        --public-endpoint <SPEC>         A publicly exposed endpoint of this Formation in the form of 'http:ROUTE=FLIGHT:PORT' (supports comma separated list, or multiple uses) [aliases: public-endpoints]
    -q, --quiet                          Suppress output at a specific level and below
        --region <REGION>                A region in which this Formation's Flights are allowed to run in (supports comma separated list, or multiple uses) (See REGION SPEC below) [default: all] [aliases: regions] [possible values: xa, asia, xc, prc, peoplesrepublicofchina, xe, europe, eu, xf, africa, xn, northamerica, namerica, xo, oceania, xq, antarctica, xs, samerica, southamerica, xu, uk, unitedkingdom, all]
    -S, --stateless                      Ignore local state files, do not read from or write to them
    -v, --verbose                        Display more verbose output
    -V, --version                        Print version information

INLINE FLIGHT OPTIONS:
        --flight-api-permission         This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime [aliases: flight-api-permissions]
        --flight-architecture <ARCH>    The architectures this flight is capable of running on. No value means it will be auto detected from the image definition [aliases: flight-arch, flight-arches, flight-architectures] [possible values: AMD64, ARM64]
        --flight-image <SPEC>           The container image registry reference that this Flight will use (See IMAGE SPEC below) [aliases: img]
        --flight-maximum <NUM>          The maximum number of container instances that should ever be running (default: infinite) [aliases: flight-max]
        --flight-minimum <NUM>          The minimum number of container instances that should ever be running [default: 1] [aliases: flight-min]
        --flight-name <STRING>          A human readable name for the Flight (must be unique within any Formation it is a part of) if omitted a pseudo random name will be assigned
        --flight-no-maximum             The maximum number of container instances that should ever be running (default: infinite) [aliases: flight-no-max]

FLIGHT SPEC

    The Flight may be specified in one of the following ways

    FLIGHT_SPEC := NAME | ID | @path | @-
    @path       := PATH is an existing file with a Flight definition in JSON format
    @-          := STDIN will be read for a Flight definition in JSON format

    NOTE that when using @- only one Flight definition may be provided via STDIN

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
$ seaplane formation create --help
seaplane-formation-create [PKGVER]
Create a Seaplane Formation

When using the inline-flight-options (--flight-*) all options apply only to a single flight. Other
Flights may be specified using the `--flight` flag, but those are totally independent of the
`--flight-*` specified Flight.

USAGE:
    seaplane formation create [OPTIONS]
    seaplane formation create --flight=SPEC... [FORMATION CFG OPTIONS]
    seaplane formation create --flight-image=SPEC [INLINE FLIGHT OPTIONS] [FORMATION CFG OPTIONS]

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with your account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

        --affinity <NAME|ID>
            A Formation that this Formation has an affinity for.
            
            This is a hint to the scheduler to place containers running in each of these
            formations "close" to eachother (for some version of close including but
            not limited to latency).
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: affinities]

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

        --connection <NAME|ID>
            A Formations that this Formation is connected to.
            
            Two formations can communicate over their formation endpoints (the endpoints configured via
            --formation-endpoints) if and only if both formations opt in to that connection (list
            each other in their connections map)
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: connections]

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
            [possible values: xa, asia, xc, prc, peoplesrepublicofchina, xe, europe, eu, xf, africa, xn, northamerica, namerica, xo, oceania, xq, antarctica, xs, samerica, southamerica, xu, uk, unitedkingdom, all]

    -F, --fetch
            Fetch remote definitions prior to creating to check for conflicts (by default only local state is considered)

        --flight <SPEC>
            A Flight to add to this formation in the form of ID|NAME|@path|@- (See FLIGHT SPEC below)
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: flights]

        --flight-endpoint <SPEC>
            A privately exposed endpoint of this Formation (only exposed to other Flights within this same Formation)
            
            Flight Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where
            
            PROTO  := http | https | tcp | udp
            TARGET := ROUTE | PORT
            ROUTE  := with PROTO http, and HTTP URL route
            PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
            FLIGHT := NAME or ID
            PORT   := Network Port (0-65535)
            
            This describes where traffic arriving at this Formation's domain URL from within this Formation's
            private network should be sent.
            
            For example, consider:
            
            $ seaplane formation edit Foo --flight-endpoint=udp:1234=baz:4321
            
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

        --formation-endpoint <SPEC>
            A privately exposed endpoint of this Formation (only expose to other Formations)
            
            Formation Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where
            
            PROTO  := http | https | tcp | udp
            TARGET := ROUTE | PORT
            ROUTE  := with PROTO http, and HTTP URL route
            PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
            FLIGHT := NAME or ID
            PORT   := Network Port (0-65535)
            
            This describes where traffic arriving at this Formation's domains URL from the private network
            should be sent.
            
            For example, consider:
            
            $ seaplane formation edit Foo --formation-endpoint=tcp:22=baz:2222
            
            Would mean, route all traffic arriving to the 'Foo' Formation's domain URL on TCP/22 from the
            private network to the the Formation's Flight named 'baz' on port '2222'. The PROTO of the incoming
            traffic will be used for the PROTO of the outgoing traffic to FLIGHT
            
            Note 'https' can be used interchangeably with 'http' for convenience sake. It does NOT however
            require the traffic actually be HTTPS. Here 'http' (or convenience 'https') simply means "Traffic
            using the HTTP" protocol.
            
            Multiple items can be passed as a comma separated list, or by using the argument
            multiple times.
            
            [aliases: formation-endpoints]

        --grounded
            This Formation configuration should be deployed but NOT set as active (requires a formation configuration)
            
            [aliases: no-active]

    -h, --help
            Print help information

        --launch
            This Formation configuration should be deployed and set as active right away (requires a formation configuration)
            
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
            A publicly exposed endpoint of this Formation
            
            Public Endpoints take the form 'http:{ROUTE}={FLIGHT}:{PORT}'. Where
            
            ROUTE  := An HTTP URL route
            FLIGHT := NAME or ID
            PORT   := Network Port (0-65535)
            
            This describes which Flight and port should serve the HTTP traffic arriving at this Formation's
            domain URL using the specified route.
            
            For example, consider:
            
            $ seaplane formation edit Foo --public-endpoint=http:/foo/bar=baz:1234
            
            Would mean, all HTTP traffic from the public internet hitting the route '/foo/bar' on the 'Foo'
            Formation's domain should be directed to this Formation's Flight named 'baz' on port '1234'
            
            In the future, support for other protocols may be added alongside 'http'
            
            Note 'https' can be used interchangeably with 'http' for convenience sake. It does NOT however
            require the traffic actually be HTTPS. Here 'http' (or convenience 'https') simply means "Traffic
            using the HTTP" protocol.
            
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
            [possible values: xa, asia, xc, prc, peoplesrepublicofchina, xe, europe, eu, xf, africa, xn, northamerica, namerica, xo, oceania, xq, antarctica, xs, samerica, southamerica, xu, uk, unitedkingdom, all]

    -S, --stateless
            Ignore local state files, do not read from or write to them

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

INLINE FLIGHT OPTIONS:
        --flight-api-permission
            This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime
            
            [aliases: flight-api-permissions]

        --flight-architecture <ARCH>
            The architectures this flight is capable of running on. No value means it will be auto detected from the image definition
            
            [aliases: flight-arch, flight-arches, flight-architectures]
            [possible values: AMD64, ARM64]

        --flight-image <SPEC>
            The container image registry reference that this Flight will use (See IMAGE SPEC below)
            
            All image references using the 'registry.seaplanet.io' registry may omit the domain portions of the
            image reference as it is implied. For example, 'registry.seaplanet.io/USER/myimage:latest' can be
            supplied simply as 'USER/myimage:latest'
            
            NOTE at this time the only registry supported is registry.seaplanet.io. In the future when other
            registries are supported, you must specify the full registry domain and path if using those
            alternate registries in order to properly reference your image.
            
            [aliases: img]

        --flight-maximum <NUM>
            The maximum number of container instances that should ever be running (default: infinite)
            
            [aliases: flight-max]

        --flight-minimum <NUM>
            The minimum number of container instances that should ever be running
            
            [default: 1]
            [aliases: flight-min]

        --flight-name <STRING>
            A human readable name for the Flight (must be unique within any Formation it
            
            Rules for a valid name are as follows:
            
              - may only include 0-9, a-z, A-Z, and '-' (hyphen)
              - hyphens ('-') may not be repeated (i.e. '--')
              - no more than three (3) total hyphens
              - the total length must be <= 27
            
            Some of these restrictions may be lifted in the future.

        --flight-no-maximum
            The maximum number of container instances that should ever be running (default: infinite)
            
            [aliases: flight-no-max]

FLIGHT SPEC

    The Flight may be specified in one of the following ways

    FLIGHT_SPEC := NAME | ID | @path | @-
    @path       := PATH is an existing file with a Flight definition in JSON format
    @-          := STDIN will be read for a Flight definition in JSON format

    NOTE that when using @- only one Flight definition may be provided via STDIN

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
