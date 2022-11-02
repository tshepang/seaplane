# Seaplane CLI

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Dependency Status][deps-image]][deps-link]


The Seaplane CLI tool allows one to interact with our services through the
command line and is the preferred method of interacting with the services.

We also have SDKs for various languages (in fact this CLI is built on top of
the Seaplane Rust SDK).

<!-- vim-markdown-toc GFM -->

* [Prerequisites](#prerequisites)
* [Installation](#installation)
    * [Install from a Github Release](#install-from-a-github-release)
    * [Compile from crates.io](#compile-from-cratesio)
    * [Compile from Source](#compile-from-source)
* [Usage of the Seaplane CLI Tool](#usage-of-the-seaplane-cli-tool)
    * [Configure Your API Key](#configure-your-api-key)
        * [Security of `SEAPLANE_API_KEY` Environment Variable](#security-of-seaplane_api_key-environment-variable)
        * [Security of `--api-key` CLI Flag](#security-of---api-key-cli-flag)
        * [Storing the API key in the Configuration File](#storing-the-api-key-in-the-configuration-file)
    * [Test!](#test)
* [License](#license)

<!-- vim-markdown-toc -->

## Prerequisites

Before using the SDK you'll want to ensure you've completed a few steps:

- You've Signed up for a Seaplane Account (we're currently in private beta so
  this means you've received an invite link, and followed that link to create
  your account)
- You copied the API given to you after account sign-up (which can also be
  found at via our [Flightdeck][flightdeck])

All service backends require a valid API Key to be used.

## Installation

The Seaplane CLI tool `seaplane` is a single binary that must be placed
somewhere in your `$PATH`. One can either download pre-compiled binaries from
the [Release Page][releases] or compile it from source.

### Install from a Github Release

The first step is to download the tool, which is a single static binary, from
our [Github Releases][releases] page.

The CLI tool is supported on both x86_64, and arm64 for Linux and macOS, as
well as x86_64 Windows. Ensure you download the appropriate archive for your
system. This guide will be using a macOS variant as a demonstration.

We'll assume the download was saved in the Downloads directory of your home
folder (`~/Downloads`).

We need to extract the binary and place it somewhere pointed to by your `$PATH`
variable. On macOS and Linux `/usr/local/bin/` is a good location.

**NOTE:** You'll need to replace `$ARCH` and `$VERSION` with whichever
architecture and version you downloaded from the release page.

```console
$ cd ~/Downloads
$ sudo unzip ./seaplane-$VERSION-$ARCH.zip -d /usr/local/bin/
```

### Compile from crates.io

Ensure you have a [Rust toolchain installed][rustup].

```
$ cargo install seaplane-cli
```

### Compile from Source

Ensure you have a [Rust toolchain installed][rustup].

```
$ git clone https://github.com/seaplane-io/seaplane
$ cd seaplane/seaplane-cli/
$ cargo build --release
$ sudo cp ../target/release/seaplane /usr/local/bin/
```

## Usage of the Seaplane CLI Tool

We can ensure that installation worked by typing `seaplane` which should
display a help message similar to below.

It's OK if the help message looks a little bit different, we're constantly
iterating and trying to improve our product!

```console
$ seaplane
seaplane v0.1.0 (f9f6dedab8)
Seaplane IO, Inc.

USAGE:
    seaplane [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    account             Operate on your Seaplane account, including access tokens [aliases: acct]
    flight              Operate on Seaplane Flights (logical containers), which are the core component of Formations
    formation           Operate on Seaplane Formations
    help                Print this message or the help of the given subcommand(s)
    image
    init                Create the Seaplane directory structure at the appropriate locations
    license
    shell-completion    Generate shell completion script files for seaplane
```

Success!

### Configure Your API Key

The final setup step is to ensure `seaplane` knows about our API key. We can do
this in a few different ways:

- Set `api-key = "..."` in our configuration file
- Set the `SEAPLANE_API_KEY` environment variable
- Use the `--api-key` CLI flag

Which you choose depends on your preferences and needs. Each has different
security and override-ability considerations.

Each of these options overrides the options above, meaning if you set an API key
in your configuration file, it can be overridden by setting the environment
variable or using the command line flag. This is helpful if you need to change
your API key for just a few invocations.

We generally recommend the configuration file, when that's possible in your
situation.

#### Security of `SEAPLANE_API_KEY` Environment Variable

When the `seaplane` process executes, it's possible for some other processes to
see environment that was given to `seaplane`. Generally this requires elevated
privileges, but that may not always be the case.

#### Security of `--api-key` CLI Flag

Like the environment variable when the `seaplane` process executes, it's
possible for some other processes to see command line flags given to
`seaplane`. Generally this requires elevated privileges, but that may not
always be the case.

However, unlike the environment variable the `--api-key` flag supports a more
secure option of using the value `-` which means "read the API key from STDIN"
which is generally considered secure, and not viewable by other processes on
the same system.

For example, if the API key was stored in a file called `my_api_key.txt` and
using the short flag of `--api-key` of `-A`:

```console
$ cat my_api_key.txt | seaplane -A-
```

#### Storing the API key in the Configuration File

We can use `seaplane account login` to store our API key in the configuration
file. One could also just write the API key to the configuration file manually,
however then you have to *find* the configuration file, make sure it properly
formatted, etc. It's easier to just let us handle it!

You will be prompted to paste your API key which will be stored in the
appropriate location of the configuration file.

```console
$ seaplane account login
Enter your API key below.
(hint: it can be found by visiting https://flightdeck.cplane.cloud/)

InlifethevisiblesurfaceoftheSpermWhaleisnottheleastamongthemanymarvelshepresents
Successfully saved the API key!
```

### Test!

Now that we have a shiny new API key installed, we can make sure it's working!

For this we'll perform silly test of asking our Access Token endpoint for a new
access token. In general you'll never need to interact with this feature
directly. However, internally especially in our [SDK] this is used quite
heavily. If this works, we know everything is installed correctly.

```console
$ seaplane account token
eyJ0eXAiOiJKV1QiLCJraWQiOiJhdXRoLXNpZ24ta2V5LTEiLCJhbGciOiJFZERTQSJ9.eyJpc3MiOi
Jpby5zZWFwbGFuZXQuZmxpZ2h0ZGVjayIsImF1ZCI6ImlvLnNlYXBsYW5ldCIsInN1YiI6IklubGlmZ
XRoZXZpc2libGVzdXJmYWNlb2Z0aGVTcGVybVdoYWxlaXNub3R0aGVsZWFzdGFtb25ndGhlbWFueW1h
cnZlbHNoZXByZXNlbnRzIiwiaWF0IjoxNjQ2NzUzODIwLCJleHAiOjE2NDY3NTM4ODAsInRlbmFudCI
6IklubGlmZXRoZXZpc2libGVzdXJmYWNlb2Z0aGVTcGVybVdoYWxlaXNub3R0aGVsZWFzdGFtb25ndG
hlbWFueW1hcnZlbHNoZXByZXNlbnRzIiwic2NvcGUiOiIifQ.epUyBWDiU2N6C7CBM7gnZPqoixd_ZH
dB8Khn_1BKwnjNxJaIba9PC9YTuDwYaFVs17aCWhY-oRDPpmo87YBrDQ
```

The access token is just a JWT and allows access to our public APIs derived
from your API key. These tokens are only valid for a very short period of time.
The token above should be *long* expired by the time you read this.

Congratulations! You now have a working Seaplane CLI ready to run some
fantastic workloads!

## License

Licensed under the Apache License, Version 2.0, [LICENSE]. Copyright 2022 Seaplane IO, Inc.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/seaplane-cli.svg
[crate-link]: https://crates.io/crates/seaplane-cli
[deps-image]: https://deps.rs/repo/github/seaplane-io/seaplane-cli/status.svg
[deps-link]: https://deps.rs/crate/seaplane-cli
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg

[//]: # (Links)

[flightdeck]: https://flightdeck.cplane.cloud/
[TOML]: https://toml.io
[rustup]: https://rustup.rs
[docs/CONFIGURATION_SPEC.md]: https://github.com/seaplane-io/seaplane/blob/main/docs/CONFIGURATION_SPEC.md
[SDK]: https://github.com/seaplane-io/seaplane/tree/main/seaplane
[releases]: https://github.com/seaplane-io/seaplane/releases
[LICENSE]: https://github.com/seaplane-io/seaplane/blob/main/LICENSE
