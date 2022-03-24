//
// The below tests are, "Do the CLI arguments we've set up have the semantics we expect"
//
// Additionally, we don't care about the output, just whether or not a run failed. These tests
// ensure as we change the CLI it maintains the same semantics
//
// Also note these runs don't actually do anything. They just parse the CLI so we don't need to
// mock anything or such.
//

macro_rules! cli {
    ($argv:expr) => {{
        seaplane_cli::test_run(const_format::concatcp!("seaplane ", $argv).split(" "))
    }};
}

#[test]
fn seaplane() {
    // The help is displayed
    assert!(cli!("").is_err());

    // For the OK tests we have to use a subcommand, so we pick init which was chosen by fair
    // diceroll.
    // --color and --no-color can override
    assert!(cli!("init --color=always --no-color").is_ok());
    // --quiet can stack
    assert!(cli!("init -qqq").is_ok());
    // --verbose can stack
    assert!(cli!("init -vvv").is_ok());
    // --api-key accepts '-' as a value
    assert!(cli!("init -A-").is_ok());
    // valid --color values
    assert!(cli!("init --color=always").is_ok());
    assert!(cli!("init --color=ansi").is_ok());
    assert!(cli!("init --color=auto").is_ok());
    assert!(cli!("init --color=never").is_ok());
    // --color values are not case sensitive
    assert!(cli!("init --color=Always").is_ok());
    assert!(cli!("init --color=ALWAYS").is_ok());
    assert!(cli!("init --color=AlWaYS").is_ok());
    // invalid --color values
    assert!(cli!("init --color=ishmael").is_err());
}

#[test]
fn seaplane_license() {
    assert!(cli!("license").is_ok());
    assert!(cli!("license --third-party").is_ok());
}

#[test]
fn seaplane_init() {
    assert!(cli!("init").is_ok());
    assert!(cli!("init --force").is_ok());
    // Force and overwrite can be used together
    assert!(cli!("init --force --overwrite=all").is_ok());

    // Valid overwrites
    assert!(cli!("init --overwrite=all").is_ok());
    assert!(cli!("init --overwrite=config").is_ok());
    assert!(cli!("init --overwrite=flights").is_ok());
    assert!(cli!("init --overwrite=formations").is_ok());

    // Invalid overwrite
    assert!(cli!("init --overwrite=foo").is_err());
    // Invalid overwrite with --force is still error
    assert!(cli!("init --force --overwrite=foo").is_err());
}

#[test]
fn seaplane_account() {
    // help displayed
    assert!(cli!("account").is_err());
}

#[test]
fn seaplane_account_token() {
    // The API key is required, but we manually check that and error if it's not present, so we
    // can't check it in the CLI tests

    // Give the API key
    assert!(cli!("account token -Afoo").is_ok());
    assert!(cli!("account -Afoo token").is_ok());
    assert!(cli!("-Afoo account token").is_ok());
}

#[test]
fn seaplane_account_login() {
    // API key required or it hangs so we can't test just the bare subcommand
    // Give the API key
    assert!(cli!("account login -Afoo").is_ok());
    assert!(cli!("account -Afoo login").is_ok());
    assert!(cli!("-Afoo account login").is_ok());
}

#[test]
fn seaplane_shell_completion() {
    // requires a SHELL
    assert!(cli!("shell-completion").is_err());
    // Invalid SHELL
    assert!(cli!("shell-completion bash").is_ok());
    // Give the SHELL
    assert!(cli!("shell-completion bash").is_ok());
    assert!(cli!("shell-completion zsh").is_ok());
    assert!(cli!("shell-completion powershell").is_ok());
    assert!(cli!("shell-completion elvish").is_ok());
    assert!(cli!("shell-completion fish").is_ok());
    // Shells are not case sensitive
    assert!(cli!("shell-completion Fish").is_ok());
    assert!(cli!("shell-completion FISH").is_ok());
    assert!(cli!("shell-completion fIsH").is_ok());
    // Invalid SHELL
    assert!(cli!("shell-completion jibberish").is_err());
}

#[test]
fn seaplane_flight() {
    // help is displayed
    assert!(cli!("flight").is_err());
}

#[test]
fn seaplane_flight_common() {
    // Because we use a common CLI set, we will use the copy command to test those common args
    // and then we don't need to re-test those args in each subcommand that simply reuses the
    // common arguments

    // aliases
    assert!(cli!("flight copy foo --maximum 2").is_ok());
    assert!(cli!("flight copy foo --max 2").is_ok());

    assert!(cli!("flight copy foo --minimum 2").is_ok());
    assert!(cli!("flight copy foo --min 2").is_ok());

    assert!(cli!("flight copy foo --api-permission").is_ok());
    assert!(cli!("flight copy foo --api-permissions").is_ok());

    assert!(cli!("flight copy foo --no-api-permission").is_ok());
    assert!(cli!("flight copy foo --no-api-permissions").is_ok());

    assert!(cli!("flight copy foo --architecture=amd64").is_ok());
    assert!(cli!("flight copy foo --architectures=amd64").is_ok());
    assert!(cli!("flight copy foo --arch=amd64").is_ok());
    assert!(cli!("flight copy foo --arches=amd64").is_ok());
    // --architecture case insensitive
    assert!(cli!("flight copy foo --arches=amd64").is_ok());
    assert!(cli!("flight copy foo --arches=AMD64").is_ok());
    assert!(cli!("flight copy foo --arches=AmD64").is_ok());

    // --architecture multiple items
    assert!(cli!("flight copy foo --arch=amd64,arm64").is_ok());
    assert!(cli!("flight copy foo --arch=amd64,arm64 --arch=amd64,arm64").is_ok());
    assert!(cli!("flight copy foo --arch=amd64 --arch=amd64").is_ok());
    assert!(cli!("flight copy foo --arch=amd64,arm64 --arch=amd64").is_ok());
    // cannot be multiple without comma or second use
    assert!(cli!("flight copy foo --arch amd64 arm64").is_err());

    // valid arches
    assert!(cli!("flight copy foo --arch=amd64,arm64").is_ok());
    // invalid arches
    assert!(cli!("flight copy foo --arch=pequod").is_err());

    // --no-* doesn't conflict
    assert!(cli!("flight copy foo --no-max --max=2").is_ok());
    assert!(cli!("flight copy foo --no-api-permission --api-permission").is_ok());
}

#[test]
fn seaplane_flight_copy() {
    // requires a NAME|ID
    assert!(cli!("flight copy").is_err());
    // provide a NAME|ID
    assert!(cli!("flight copy foo").is_ok());
    // invalid NAME|ID
    assert!(cli!("flight copy way-too-many-hyphens-to-pass-validation").is_err());

    // clone is an alias
    assert!(cli!("flight clone foo").is_ok());
}

#[test]
fn seaplane_flight_create() {
    // provide an --image
    assert!(cli!("flight create --image ahab/alpine:latest").is_ok());
    // invalid name
    assert!(cli!(
        "flight --image ahab/alpine:latest --name create way-too-many-hyphens-to-pass-validation"
    )
    .is_err());

    // add is an alias
    assert!(cli!("flight add --image ahab/alpine:latest").is_ok());
}

#[test]
fn seaplane_flight_edit() {
    // requires a NAME|ID
    assert!(cli!("flight edit").is_err());
    // provide a NAME|ID
    assert!(cli!("flight edit foo").is_ok());
    // invalid NAME|ID
    assert!(cli!("flight edit way-too-many-hyphens-to-pass-validation").is_err());
}

#[test]
fn seaplane_flight_delete() {
    // requires a NAME|ID
    assert!(cli!("flight delete").is_err());
    // provide a NAME|ID
    assert!(cli!("flight delete foo").is_ok());
    // invalid NAME|ID
    assert!(cli!("flight delete way-too-many-hyphens-to-pass-validation").is_err());
    // --all and --exact conflict
    assert!(cli!("flight delete foo --all --exact").is_err());

    // aliases
    assert!(cli!("flight del foo").is_ok());
    assert!(cli!("flight remove foo").is_ok());
    assert!(cli!("flight rm foo").is_ok());
}

#[test]
fn seaplane_flight_list() {
    assert!(cli!("flight list").is_ok());

    // aliases
    assert!(cli!("flight ls").is_ok());
}

#[test]
fn seaplane_formation_common() {
    // Because we use a common CLI set, we will use the create command to test those common args
    // and then we don't need to re-test those args in each subcommand that simply reuses the
    // common arguments

    // valid name
    assert!(cli!("formation create --name foo").is_ok());
    // invalid name
    assert!(cli!("formation create --name way-too-many-hyphens-to-pass-validation").is_err());

    // launch is the default, but should not error when used
    assert!(cli!("formation create --launch").is_ok());
    // Same with it's alias
    assert!(cli!("formation create --active").is_ok());
    // Overrides
    assert!(cli!("formation create --launch --no-launch").is_ok());
    assert!(cli!("formation create --active --no-active").is_ok());
    assert!(cli!("formation create --deploy --no-launch").is_ok());
    assert!(cli!("formation create --launch --no-deploy").is_ok());

    // flight
    // valid (@path requires a valid file...so we're not testing that and relying on the unit
    // tests for that functionality)
    assert!(cli!("formation create --flight foo").is_ok());
    assert!(cli!("formation create --flight @-").is_ok());
    // invalid
    assert!(cli!("formation create --flight way-too-many-hyphens-to-pass-validation").is_err());
    // multiples only with commas or multiple uses
    assert!(cli!("formation create --flight=foo,bar,baz").is_ok());
    assert!(cli!("formation create --flight=foo --flight=bar,baz").is_ok());
    assert!(cli!("formation create --flight=foo,bar --flight=baz").is_ok());
    assert!(cli!("formation create --flight foo bar baz").is_err());

    // affinity
    // valid
    assert!(cli!("formation create --affinity foo").is_ok());
    // invalid
    assert!(cli!("formation create --affinity way-too-many-hyphens-to-pass-validation").is_err());
    // multiples only with commas or multiple uses
    assert!(cli!("formation create --affinity=foo,bar,baz").is_ok());
    assert!(cli!("formation create --affinity=foo --affinity=bar,baz").is_ok());
    assert!(cli!("formation create --affinity=foo,bar --affinity=baz").is_ok());
    assert!(cli!("formation create --affinity foo bar baz").is_err());
    // alias
    assert!(cli!("formation create --affinities foo").is_ok());

    // connection
    // valid
    assert!(cli!("formation create --connection foo").is_ok());
    // invalid
    assert!(cli!("formation create --connection way-too-many-hyphens-to-pass-validation").is_err());
    // multiples only with commas or multiple uses
    assert!(cli!("formation create --connection=foo,bar,baz").is_ok());
    assert!(cli!("formation create --connection=foo --connection=bar,baz").is_ok());
    assert!(cli!("formation create --connection=foo,bar --connection=baz").is_ok());
    assert!(cli!("formation create --connection foo bar baz").is_err());
    // alias
    assert!(cli!("formation create --connections foo").is_ok());

    // provider
    // valid
    assert!(cli!("formation create --provider=Aws,Azure,DigitalOcean,Equinix,Gcp,All").is_ok());
    // invalid
    assert!(cli!("formation create --provider=carpet").is_err());
    // Case insensitive
    assert!(cli!("formation create --provider=AWS").is_ok());
    assert!(cli!("formation create --provider=aws").is_ok());
    assert!(cli!("formation create --provider=aWs").is_ok());
    // multiples only with commas or multiple uses
    assert!(cli!("formation create --provider=Aws,Azure --provider=DigitalOcean").is_ok());
    assert!(cli!("formation create --provider=Aws --provider=DigitalOcean,Azure").is_ok());
    assert!(cli!("formation create --provider=Aws --provider=DigitalOcean").is_ok());
    assert!(cli!("formation create --provider Aws Azure DigitalOcean").is_err());
    // alias
    assert!(cli!("formation create --providers=Aws,Azure,DigitalOcean,Equinix,Gcp,All").is_ok());

    // exlucde provider
    // valid
    assert!(
        cli!("formation create --exclude-provider=Aws,Azure,DigitalOcean,Equinix,Gcp,All").is_ok()
    );
    // invalid
    assert!(cli!("formation create --exclude-provider=carpet").is_err());
    // Case insensitive
    assert!(cli!("formation create --exclude-provider=AWS").is_ok());
    assert!(cli!("formation create --exclude-provider=aws").is_ok());
    assert!(cli!("formation create --exclude-provider=aWs").is_ok());
    // multiples only with commas or multiple uses
    assert!(
        cli!("formation create --exclude-provider=Aws,Azure --exclude-provider=DigitalOcean")
            .is_ok()
    );
    assert!(
        cli!("formation create --exclude-provider=Aws --exclude-provider=DigitalOcean,Azure")
            .is_ok()
    );
    assert!(
        cli!("formation create --exclude-provider=Aws --exclude-provider=DigitalOcean").is_ok()
    );
    assert!(cli!("formation create --exclude-provider Aws Azure DigitalOcean").is_err());
    // alias
    assert!(
        cli!("formation create --exclude-providers=Aws,Azure,DigitalOcean,Equinix,Gcp,All").is_ok()
    );

    // region
    // valid
    assert!(
        cli!("formation create --region=XA,Asia,XC,PRC,PeoplesRepublicofChina,XE,Europe,EU,XF,Africa,XN,NorthAmerica,NAmerica,XO,Oceania,XQ,Antarctica,XS,SAmerica,SouthAmerica,XU,UK,UnitedKingdom,All")
            .is_ok()
    );
    // invalid
    assert!(cli!("formation create --region=carpet").is_err());
    // Case insensitive
    assert!(cli!("formation create --region=Oceania").is_ok());
    assert!(cli!("formation create --region=oceania").is_ok());
    assert!(cli!("formation create --region=OcEanIa").is_ok());
    // multiples only with commas or multiple uses
    assert!(cli!("formation create --region=XA,prc --region=europe").is_ok());
    assert!(cli!("formation create --region=eu --region=xn,xs").is_ok());
    assert!(cli!("formation create --region=uk --region=namerica").is_ok());
    assert!(cli!("formation create --region xa xc xf").is_err());
    // alias
    assert!(cli!("formation create --regions=XA,Asia,XC").is_ok());

    // exclude region
    // valid
    assert!(
        cli!("formation create --exclude-region=XA,Asia,XC,PRC,PeoplesRepublicofChina,XE,Europe,EU,XF,Africa,XN,NorthAmerica,NAmerica,XO,Oceania,XQ,Antarctica,XS,SAmerica,SouthAmerica,XU,UK,UnitedKingdom,All")
            .is_ok()
    );
    // invalid
    assert!(cli!("formation create --exclude-region=carpet").is_err());
    // Case insensitive
    assert!(cli!("formation create --exclude-region=Oceania").is_ok());
    assert!(cli!("formation create --exclude-region=oceania").is_ok());
    assert!(cli!("formation create --exclude-region=OcEanIa").is_ok());
    // multiples only with commas or multiple uses
    assert!(cli!("formation create --exclude-region=XA,prc --exclude-region=europe").is_ok());
    assert!(cli!("formation create --exclude-region=eu --exclude-region=xn,xs").is_ok());
    assert!(cli!("formation create --exclude-region=uk --exclude-region=namerica").is_ok());
    assert!(cli!("formation create --exclude-region xa xc xf").is_err());
    // alias
    assert!(cli!("formation create --exclude-regions=XA,Asia,XC").is_ok());

    // public endpoint (we don't try to enumerate valid endpoints because that should happen
    // in unit tests, just that *some* form of validation happens along with any semantics such
    // as multiples, etc.)
    // valid
    assert!(cli!("formation create --public-endpoint=http:foo/bar=baz:123").is_ok());
    assert!(cli!("formation create --exclude-region xa xc xf").is_err());
    // alias
    assert!(cli!("formation create --exclude-regions=XA,Asia,XC").is_ok());

    // public endpoint (we don't try to enumerate valid endpoints because that should happen
    // in unit tests, just that *some* form of validation happens along with any semantics such
    // as multiples, etc.)
    // valid
    assert!(cli!("formation create --public-endpoint=http:foo/bar=baz:123").is_ok());
    // invalid
    assert!(cli!("formation create --public-endpoint=carpet").is_err());
    // multiples only with commas or multiple uses
    assert!( cli!("formation create --public-endpoint=http:foo/bar=baz:123,http:baz/qux=nom:432 --public-endpoint=http:/=que:5432") .is_ok());
    assert!( cli!("formation create --public-endpoint=http:foo/bar=baz:123 --public-endpoint=http:/=que:5432,http:baz/qux=nom:432") .is_ok());
    assert!(cli!(
        "formation create --public-endpoint=http:foo/bar=baz:123 --public-endpoint=http:/=que:5432"
    )
    .is_ok());
    assert!( cli!("formation create --public-endpoint=http:foo/bar=baz:123 http:baz/qux=nom:432 http:/=que:5432") .is_err());
    // alias
    assert!(cli!("formation create --public-endpoints=http:foo/bar=baz:123").is_ok());

    // formation endpoint (we don't try to enumerate valid endpoints because that should happen
    // in unit tests, just that *some* form of validation happens along with any semantics such
    // as multiples, etc.)
    // valid
    assert!(cli!("formation create --formation-endpoint=tcp:22=baz:123").is_ok());
    // invalid
    assert!(cli!("formation create --formation-endpoint=carpet").is_err());
    // multiples only with commas or multiple uses
    assert!( cli!("formation create --formation-endpoint=http:foo/bar=baz:123,udp:987=nom:432 --formation-endpoint=http:/=que:5432") .is_ok());
    assert!( cli!("formation create --formation-endpoint=tcp:123=baz:123 --formation-endpoint=http:/=que:5432,udp:876=nom:432") .is_ok());
    assert!( cli!("formation create --formation-endpoint=udp:1234=baz:123 --formation-endpoint=http:/=que:5432") .is_ok());
    assert!( cli!("formation create --formation-endpoint=http:foo/bar=baz:123 tcp:baz/qux=nom:432 http:/=que:5432") .is_err());
    // alias
    assert!(cli!("formation create --formation-endpoints=udp:1234=baz:123").is_ok());

    // flight endpoint (we don't try to enumerate valid endpoints because that should happen
    // in unit tests, just that *some* form of validation happens along with any semantics such
    // as multiples, etc.)
    // valid
    assert!(cli!("formation create --flight-endpoint=tcp:22=baz:123").is_ok());
    // invalid
    assert!(cli!("formation create --flight-endpoint=carpet").is_err());
    // multiples only with commas or multiple uses
    assert!( cli!("formation create --flight-endpoint=http:foo/bar=baz:123,udp:987=nom:432 --flight-endpoint=http:/=que:5432") .is_ok());
    assert!( cli!("formation create --flight-endpoint=tcp:123=baz:123 --flight-endpoint=http:/=que:5432,udp:876=nom:432") .is_ok());
    assert!(cli!(
        "formation create --flight-endpoint=udp:1234=baz:123 --flight-endpoint=http:/=que:5432"
    )
    .is_ok());
    assert!( cli!("formation create --flight-endpoint=http:foo/bar=baz:123 tcp:baz/qux=nom:432 http:/=que:5432") .is_err());
    // alias
    assert!(cli!("formation create --flight-endpoints=udp:1234=baz:123").is_ok());
}

#[test]
fn seaplane_formation_list() {
    assert!(cli!("formation list").is_ok());

    // aliases
    assert!(cli!("formation ls").is_ok());
}

#[test]
fn seaplane_formation_create() {
    assert!(cli!("formation create").is_ok());
    // invalid name
    assert!(cli!("formation --name create way-too-many-hyphens-to-pass-validation").is_err());

    // options
    assert!(cli!("formation create --force").is_ok());
    assert!(cli!("formation create --launch").is_ok());
    assert!(cli!("formation create --deploy").is_ok());
    // overrides
    assert!(cli!("formation create --launch --no-launch").is_ok());
    assert!(cli!("formation create --deploy --no-deploy").is_ok());
    // should be OK but not override
    assert!(cli!("formation create --launch --deploy").is_ok());

    // Using any --flight-* (minus image) requires --flight-image
    assert!(cli!("formation create --flight-name foo").is_err());
    assert!(cli!("formation create --flight-minimum 2").is_err());
    assert!(cli!("formation create --flight-maximum 2").is_err());
    assert!(cli!("formation create --flight-api-permission").is_err());
    assert!(cli!("formation create --flight-architecture amd64").is_err());
    // with --flight-image works
    assert!(cli!("formation create --flight-image foo").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-name foo").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-minimum 2").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-maximum 2").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-api-permission").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-architecture amd64").is_ok());
    // aliases
    assert!(cli!("formation create --flight-image foo --flight-min 2").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-max 2").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-api-permissions").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-architectures amd64").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-arch amd64").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-arches amd64").is_ok());
    // arch is case insensitive
    assert!(cli!("formation create --flight-image foo --flight-arch amd64").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-arch AMD64").is_ok());
    assert!(cli!("formation create --flight-image foo --flight-arch AmD64").is_ok());
    // arch multiples
    assert!(cli!("formation create --flight-image foo --flight-arch=amd64,arm64").is_ok());
    assert!(cli!(
        "formation create --flight-image foo --flight-arch=amd64,arm64 --flight-arch=amd64,arm64"
    )
    .is_ok());
    assert!(
        cli!("formation create --flight-image foo --flight-arch=amd64 --flight-arch=amd64").is_ok()
    );
    assert!(cli!(
        "formation create --flight-image foo --flight-arch=amd64,arm64 --flight-arch=amd64"
    )
    .is_ok());
    // cannot be multiple without comma or second use
    assert!(cli!("formation create --flight-image foo --flight-arch amd64 arm64").is_err());

    // add is an alias
    assert!(cli!("formation add").is_ok());
}

#[test]
fn seaplane_formation_delete() {
    // requires a NAME|ID
    assert!(cli!("formation delete").is_err());
    // provide a NAME|ID
    assert!(cli!("formation delete foo").is_ok());
    // invalid NAME|ID
    assert!(cli!("formation delete way-too-many-hyphens-to-pass-validation").is_err());
    assert!(cli!("formation delete foo --remote").is_ok());
    assert!(cli!("formation delete foo --local").is_ok());
    assert!(cli!("formation delete foo --no-remote").is_ok());
    assert!(cli!("formation delete foo --no-local").is_ok());
    assert!(cli!("formation delete foo --remote --no-remote").is_ok());
    assert!(cli!("formation delete foo --local --no-local").is_ok());
    // --all and --exact conflict
    assert!(cli!("formation delete foo --all --exact").is_err());

    // aliases
    assert!(cli!("formation del foo").is_ok());
    assert!(cli!("formation remove foo").is_ok());
    assert!(cli!("formation rm foo").is_ok());
}

#[test]
fn seaplane_formation_fetch_remote() {
    assert!(cli!("formation fetch-remote").is_ok());
    assert!(cli!("formation fetch-remote foo").is_ok());

    // aliases
    assert!(cli!("formation fetch").is_ok());
}

#[test]
fn seaplane_formation_launch() {
    // requires a NAME|ID
    assert!(cli!("formation launch").is_err());
    // provide a NAME|ID
    assert!(cli!("formation launch foo").is_ok());
    // invalid NAME|ID
    assert!(cli!("formation launch way-too-many-hyphens-to-pass-validation").is_err());
    assert!(cli!("formation launch foo --fetch").is_ok());
    assert!(cli!("formation launch foo --grounded").is_ok());
    // --all and --exact conflict
    assert!(cli!("formation launch foo --all --exact").is_err());

    // aliases
    assert!(cli!("formation start foo").is_ok());
}

#[test]
fn seaplane_formation_land() {
    // requires a NAME|ID
    assert!(cli!("formation land").is_err());
    // provide a NAME|ID
    assert!(cli!("formation land foo").is_ok());
    // invalid NAME|ID
    assert!(cli!("formation land way-too-many-hyphens-to-pass-validation").is_err());
    // --all and --exact conflict
    assert!(cli!("formation land foo --all --exact").is_err());

    // aliases
    assert!(cli!("formation stop foo").is_ok());
}

#[test]
fn seaplane_kv() {
    // requires a subcmd
    assert!(cli!("key-value").is_err());
    // provide subcmd
    assert!(cli!("key-value delete foo").is_ok());
    // aliases
    assert!(cli!("kv delete foo").is_ok());
}

#[test]
fn seaplane_kv_delete() {
    // requires a KEY
    assert!(cli!("key-value delete").is_err());
    // provide a key
    assert!(cli!("key-value delete foo").is_ok());
    // multiples
    assert!(cli!("key-value delete foo bar baz").is_ok());
    assert!(cli!("key-value delete foo,bar,baz").is_ok());
    assert!(cli!("key-value delete foo bar,baz").is_ok());
    assert!(cli!("key-value delete foo,bar baz").is_ok());

    // aliases
    assert!(cli!("key-value del foo").is_ok());
    assert!(cli!("key-value remove foo").is_ok());
    assert!(cli!("key-value rm  foo").is_ok());
}

#[test]
fn seaplane_kv_get() {
    // requires a KEY
    assert!(cli!("key-value get").is_err());
    // provide a key
    assert!(cli!("key-value get foo").is_ok());
    // multiples
    assert!(cli!("key-value get foo bar baz").is_ok());
    assert!(cli!("key-value get foo,bar,baz").is_ok());
    assert!(cli!("key-value get foo bar,baz").is_ok());
    assert!(cli!("key-value get foo,bar baz").is_ok());

    // aliases
    assert!(cli!("key-value show foo").is_ok());
}

#[test]
fn seaplane_kv_set() {
    // requires a KEY and VALUE
    assert!(cli!("key-value set").is_err());
    assert!(cli!("key-value set foo").is_err());
    // provide a valid KEY VALUE
    assert!(cli!("key-value set foo bar").is_ok());
    // multiples are not allowed
    assert!(cli!("key-value set foo bar baz qux").is_err());
}

#[test]
fn seaplane_kv_list() {
    // does not require a dir
    assert!(cli!("key-value list").is_ok());
    // can provide a dir
    assert!(cli!("key-value list foo").is_ok());
    // Multiples not supported
    assert!(cli!("key-value list foo bar").is_err());

    // aliases
    assert!(cli!("key-value ls foo").is_ok());
}
