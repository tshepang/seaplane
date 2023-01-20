```console
$ seaplane formation launch -h
Start a local Formation Plan creating a remote Formation Instance

Usage: seaplane[EXE] formation launch [OPTIONS] <NAME|ID>

Arguments:
  <NAME|ID>  The name or ID of the Formation Plan to launch and create an Instance of

Options:
  -a, --all               Launch all matching local Formation Plans even when the name or ID is ambiguous
  -v, --verbose...        Display more verbose output
  -F, --fetch             Fetch remote Formation Instances and synchronize local Plan definitions prior to attempting to launch [aliases: sync, synchronize]
  -q, --quiet...          Suppress output at a specific level and below
      --color <COLOR>     Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
      --grounded          Upload the configuration(s) defined in this local Formation Plan to Seaplane but *DO NOT* set them to active
      --no-color          Do not color output (alias for --color=never)
  -A, --api-key <STRING>  The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
  -S, --stateless         Ignore local state files, do not read from or write to them
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version

```

```console
$ seaplane formation launch --help
Start a local Formation Plan creating a remote Formation Instance

In many cases, or at least initially a local Formation Plan may only have a single
Formation Configuration. In these cases this command will set that one configuration to active
creating a remote Formation Instance with a single configuration.

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

Usage: seaplane[EXE] formation launch [OPTIONS] <NAME|ID>

Arguments:
  <NAME|ID>
          The name or ID of the Formation Plan to launch and create an Instance of

Options:
  -a, --all
          Launch all matching local Formation Plans even when the name or ID is ambiguous

  -v, --verbose...
          Display more verbose output
          
          More uses displays more verbose output
              -v:  Display debug info
              -vv: Display trace info

  -F, --fetch
          Fetch remote Formation Instances and synchronize local Plan definitions prior to attempting to launch
          
          [aliases: sync, synchronize]

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

      --grounded
          Upload the configuration(s) defined in this local Formation Plan to Seaplane but *DO NOT* set them to active

      --no-color
          Do not color output (alias for --color=never)

  -A, --api-key <STRING>
          The API key associated with a Seaplane account used to access Seaplane API endpoints
          
          The value provided here will override any provided in any configuration files.
          A CLI provided value also overrides any environment variables.
          One can use a special value of '-' to signal the value should be read from STDIN.
          
          [env: SEAPLANE_API_KEY]

  -S, --stateless
          Ignore local state files, do not read from or write to them

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
