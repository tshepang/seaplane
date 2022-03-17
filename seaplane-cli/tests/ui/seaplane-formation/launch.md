```console
$ seaplane formation launch -h
seaplane-formation-launch [PKGVER]
Start all configurations of a Formation and evenly distribute traffic between them

USAGE:
    seaplane formation launch [OPTIONS] <NAME|ID>

ARGS:
    <NAME|ID>    

OPTIONS:
    -a, --all                 Stop all matching Formations even when FORMATION is ambiguous
    -A, --api-key <STRING>    The API key associated with your account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -F, --fetch               Fetch remote Formation definitions prior to attempting to launch this Formation
        --grounded            Upload the configuration(s) to Seaplane but *DO NOT* set them to active
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information
    -x, --exact               the given FORMATION must be an exact match

```

```console
$ seaplane formation launch --help
seaplane-formation-launch [PKGVER]
Start all configurations of a Formation and evenly distribute traffic between them

    In many cases, or at least initially a Formation may only have a single Formation
    Configuration. In these cases this command will set that one configuration to active.

    Things become slightly more complex when there are multiple Formation Configurations. Let's
    look at each possibility in turn.

    "Local Only" Configs Exist:

    A "Local Only" Config is a configuration that exists in the local database, but has not (yet)
    been uploaded to the Seaplane Cloud.

    In these cases the configurations will be sent to the Seaplane Cloud, and set to active. If the
    Seaplane Cloud already has configurations for the given Formation (either active or inactive),
    these new configurations will be appended, and traffic will be balanced between any *all*
    configurations.

    "Remote Active" Configs Exist:

    A "Remote Active" Config is a configuration that the Seaplane Cloud is aware of, and actively
    sending traffic to.

    These configurations will remain active and traffic will be balanced between any *all*
    configurations.

    "Remote Inactive" Configs Exist:

    A "Remote Inactive" Config is a configuration that the Seaplane Cloud is aware of, and but not
    sending traffic to.

    These configurations will be made active. If the Seaplane Cloud already has active
    configurations for the given Formation, these newly activated configurations will be appended,
    and traffic will be balanced between any *all* configurations.

USAGE:
    seaplane formation launch [OPTIONS] <NAME|ID>

ARGS:
    <NAME|ID>
            

OPTIONS:
    -a, --all
            Stop all matching Formations even when FORMATION is ambiguous

    -A, --api-key <STRING>
            The API key associated with your account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -F, --fetch
            Fetch remote Formation definitions prior to attempting to launch this Formation

        --grounded
            Upload the configuration(s) to Seaplane but *DO NOT* set them to active

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

    -x, --exact
            the given FORMATION must be an exact match

```
