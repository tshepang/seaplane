# Getting Started with Seaplane!

This guide will walk through running your first workload on the Seaplane cloud.

We've structured this guide in increasingly complex (read *real world*)
examples.

For the absolute fastest up-and-running version, see [Quickstart](#quickstart)

<!-- vim-markdown-toc GFM -->

* [Prerequisites](#prerequisites)
* [Quickstart](#quickstart)
* [Setup](#setup)
    * [The Seaplane CLI Tool](#the-seaplane-cli-tool)
    * [Initialize our Environment](#initialize-our-environment)
        * [Setting `NO_COLOR`](#setting-no_color)
        * [Using `--no-color` or setting `--color=never`](#using---no-color-or-setting---colornever)
        * [Setting `color = "never"` in the configuration file](#setting-color--never-in-the-configuration-file)
    * [Configure Your API Key](#configure-your-api-key)
        * [Security of `SEAPLANE_API_KEY` Environment Variable](#security-of-seaplane_api_key-environment-variable)
        * [Security of `--api-key` CLI Flag](#security-of---api-key-cli-flag)
        * [Storing the API key in the Configuration File](#storing-the-api-key-in-the-configuration-file)
    * [Test!](#test)
* [Running Your Workload](#running-your-workload)
    * [Upload a Container Image](#upload-a-container-image)
    * [Crate and Launch a Formation with a Single Flight](#crate-and-launch-a-formation-with-a-single-flight)
        * [Your First Workload](#your-first-workload)
        * [Working with Local Flight Definitions](#working-with-local-flight-definitions)
        * [Working with Local Formation Definitions](#working-with-local-formation-definitions)
    * [See a Hello World Page](#see-a-hello-world-page)

<!-- vim-markdown-toc -->

## Prerequisites 

Before we begin you'll want to ensure you've completed a few steps:

- You've Signed up for a Seaplane Account (we're currently in private beta so
  this means you've received an invite link, and followed that link to create
  your account)
- You copied the API given to you after account sign-up (which can also be
  found at via our [Flightdeck])

## Quickstart

As a preview, and to show just how quick it can be to spin up a Seaplane
workload, here is what it looks like to run [`nginxdemos/hello`] on our
platform.

You'll only need:

- Our single static binary for your particular platform and architecture
- An API key you received from [Flightdeck]

```console
$ seaplane formation create \
  --api-key "mysuperspecialapikey" \
  --launch \
  --flight-image seaplane_demo/nginxdemos/hello:latest
Successfully created Seaplane directories
Successfully created Flight 'itchy-sweater' with ID '664954bb'
Successfully created Formation 'few-actor' with ID 'b67437dc'
Successfully launched Formation 'few-actor' with Configuration UUIDs:
    'd87938b6-c57d-47c4-8037-6e69026008ac'
```

To see it working:

```
$ curl https://few-actor.seaplane_demo.seaplanet.io/ | head -n 4
<!DOCTYPE html>
<html>
<head>
<title>Hello World</title>
```

To learn more about the details of what is happening in these commands, what
other possibilities exist, and how to fully manage your fleet read on!

(Also note there are other more secure ways to pass your API key!)

## Setup

Here we'll:

- download and install the Seaplane CLI tool
- Initialize our environment
- Configure our API key
- Test our setup

Let's begin!

### The Seaplane CLI Tool

The Seaplane CLI tool is the go-to way to run your workloads on the Seaplane
cloud and interact with our public APIs.

The first step is to download the tool, which is a single static binary, from
our [Github Releases] page.

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
$ sudo tar xzf ./seaplane-$VERSION-$ARCH.tar.gz -C /usr/local/bin/
```

We can ensure that it worked by simply typing `seaplane` which should display a
help message similar to below.

It's OK if the help message looks a little bit different, we're constantly
iterating and trying to improve our product!

```console
$ seaplane
seaplane v0.1.0 (f9f6dedab8)
Seaplane IO, Inc.

USAGE:
    seaplane [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -A, --api-key <STRING>    The API key associated with your account used to 
                              access Seaplane API endpoints 
			      [env: SEAPLANE_API_KEY]
        --color <COLOR>       Should the output include color? [default: auto] 
	                      [possible values: always, ansi, auto, never]
    -h, --help                Print help information
        --no-color            Do not color output (alias for --color=never)
    -q, --quiet               Suppress output at a specific level and below
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

SUBCOMMANDS:
    account             Operate on your Seaplane account, including access 
                        tokens [aliases: acct]
    config
    flight              Operate on Seaplane Flights (logical containers), 
                        which are the core component of Formations
    formation           Operate on Seaplane Formations
    help                Print this message or the help of the given 
                        subcommand(s)
    image
    init                Create the Seaplane directory structure at the 
                        appropriate locations
    license
    shell-completion    Generate shell completion script files for seaplane
```

Success!

### Initialize our Environment

This step is somewhat optional as the Seaplane tool will try to auto-initialize
for you when possible. However, since we're still learning let's go ahead and
do it manually since it will show a few key areas we can configure.

Here we will create all the necessary files and directories used by the
Seaplane CLI. These directories are platform specific and only located within
your home folder, we don't touch system files or directories!

We can initialize everything with a single command:

```console
$ seaplane init
Successfully created Seaplane directories
```

If you're curious which exact files and directories were created, you can add
the `--verbose` flag. If you've already initialized, you can safely re-run with
the `--verbose` flag just to see. We won't overwrite anything unless you tell
us to.

For example, on a mac, the output looks like this:

```console
$ seaplane init --verbose
DEBUG: Looking for configuration file at "/Users/kevin/Library/Application Support/io.Seaplane.seaplane/seaplane.toml"
DEBUG: Found configuration file "/Users/kevin/Library/Application Support/io.Seaplane.seaplane/seaplane.toml"
DEBUG: Looking for configuration file at "/Users/kevin/.config/seaplane/seaplane.toml"
DEBUG: Looking for configuration file at "/Users/kevin/.seaplane/seaplane.toml"
DEBUG: Creating directory "/Users/kevin/Library/Application Support/io.Seaplane.seaplane"
DEBUG: Creating directory "/Users/kevin/Library/Application Support/io.Seaplane.seaplane"
warn: "/Users/kevin/Library/Application Support/io.Seaplane.seaplane/seaplane.toml" already exists
(hint: use 'seaplane init --overwite=config to erase and overwrite it)

warn: "/Users/kevin/Library/Application Support/io.Seaplane.seaplane/formations.json" already exists
(hint: use 'seaplane init --overwite=formations to erase and overwrite it)

warn: "/Users/kevin/Library/Application Support/io.Seaplane.seaplane/flights.json" already exists
(hint: use 'seaplane init --overwite=flights to erase and overwrite it)

Successfully created Seaplane directories
```

If you're following along live (because the above text doesn't do it justice)
you may have noticed that last command added some color to the output.

We believe tasteful coloring of words helps draw attention to important parts
of the output, this is especially important in errors and hints.

However, not everyone has the same tastes and reasonable people can disagree.
To support this, we do our best to be respectful of each individuals
preferences. You can disable any coloring of the output in several different
ways. We'll cover three of them here:

#### Setting `NO_COLOR`

If you have the `NO_COLOR` environment variable set, your output will not
contain color. In fact, if you already had that set, you may not have even
noticed the previous message was colored at all!

This cannot be overridden by `--color` flags.

#### Using `--no-color` or setting `--color=never`

These can be set as a shell alias, or only if you want to remove color from one
particular invocation.

Setting an alias also allows you to override this choice on certain invocations
as well. For example, if you normally don't want any color, but then later
decide for a particular invocation you *do*, you can simply pass
`--color=always` which will override your alias.

#### Setting `color = "never"` in the configuration file

Another method turning off color in output for all invocations is by setting
`color = "never"` in your configuration file. The configuration file is called
`seaplane.toml` and uses the [TOML] language. It's location is platform
dependant though, which you can find by looking at the output of the `seaplane
init --verbose`. In our above example it's located at `~/Library/Application
Support/io.Seaplane.seaplane/seaplane.toml`

If we add `color = "never"` under the `[seaplane]` table, our output will no
longer contain any color.

**NOTE:** For more options see the [doc/CONFIGURATION_SPEC.md] in this repository

### Configure Your API Key

The final setup step is to ensure `seplane` knows about our API key. We can do
this in a few different ways:

- Set `api_key = "..."` in our configuration file
- Set the `SEAPLANE_API_KEY` environment variable
- Use the `--api-key` CLI flag

Which you choose depends on your preferences and needs. Each has different
security and override-ability considerations.

Each of these options overrides any option above, meaning if you set an API key
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
(hint: it can be found by visiting https://flightdeck.seaplanet.io/)

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

## Running Your Workload

In this chapter we will run our first workload on the Seaplane Cloud.

We will:

- Upload a Container Image
- Create and Launch a Formation with a single Flight
- See a hello world page

Let's get started!

### Upload a Container Image

You must first upload your container image to the Seaplane Cloud Registry.
We'll be using the [`nginxdemos/hello`]

### Crate and Launch a Formation with a Single Flight

In Seaplane a *Formation* can be thought of as your application or service.
Formations are made up of *Flights* which are logical containers. 

Using the Seaplane CLI we can create Formation and Flight definitions. These
definitions reside locally, so we can edit them, copy them, change them, pretty
much anything we need.

Once we're satisfied with the definition we can `launch` it which sends it to
the Seaplane Cloud and activates it for receiving public traffic.

We can also define and launch our definitions all in one go, if desired.

For our first command we'll do it all in one go, so you can see just how
effortlessly it can happen. Then we'll break down the components into different
definitions.

#### Your First Workload

We'll be running the `nginx-hello` container from earlier.

**NOTE:** Here `seaplane_demo/` is our Tenant ID. You'll need to replace that with your
own tenant ID.

```console
$ seaplane formation create --flight-image seaplane_demo/nginxdemos/hello:latest --launch
Successfully created Flight 'mellow-order' with ID 'd4e877b7'
Successfully created Formation 'nimble-bike' with ID '8bfe5304'
Successflly launched Formation 'nimble-bike' with Configuration UUIDs:
    'c534dc48-03b6-400b-bc19-f7d28cfd1897'
```

That's it!

Notice it created a Flight and Formation definition for us, and because we
didn't specify anything it also gave them some randomly assigned names.

If all you want is to see that it worked, skip down to the [See Hello World
Page](#see-a-hello-world-page) section below.

#### Working with Local Flight Definitions

We can see our Flight definition:

```console
$ seaplane flight list
LOCAL ID  NAME          IMAGE                             MIN  MAX  ARCH  API PERMS
d4e877b7  mellow-order  seaplane_demo/nginx-hello:latest  1    INF  auto  false
```

Notice that took a bunch of default values because we didn't specify anything
in particular.

We could create an entirely new Flight definition via the `seaplane flight
create` command, but instead let's just edit the one we've already made. Let's
say that we actually don't want to be running the `:latest` tag, we *actually*
want to pin to a specific digest. We'll use
`sha256:33d30466bb608f607a8d708d39bf13ec7a908dde1a8a8b228f7f3f4c6a4d1bdf` for
this example.

**SIDE NOTE:** Even when you specify `:latest` Seaplane pins your containers to
the last digest at that point so that your Flights don't change out from under
you. The example commands are somewhat contrived and purely to demonstrate the
CLI and how to use it.

```console
$ seaplane flight edit d4e8 --image seaplane_demo/nginxdemos/hello@sha256:33d30466bb608f607a8d708d39bf13ec7a908dde1a8a8b228f7f3f4c6a4d1bdf

$ seaplane flight list
LOCAL ID  NAME              IMAGE                                                                                                   MIN  MAX  ARCH  API PERMS
d4e877b7  mellow-order      seaplane_demo/nginxdemos/hello@sha256:33d30466bb608f607a8d708d39bf13ec7a908dde1a8a8b228f7f3f4c6a4d1bdf  1    INF  auto  false
```

**IMPORANT:** This edit *only affects our local flight definition*! Nothing has
changed to our Formation running in the Seaplane Cloud.

Notice we used part of the `LOCAL ID` to reference our Flight. We could have
also done so by unambiguous partial name, or full name if we desired.

Hmm. You know what? Actually, I think we do want another Flight defined that
points to the `:latest` tag. We can copy `mellow-order` and just make that one
change.

```console
$ seaplane flight copy mellow-order --image seaplane_demo/nginxdemos/hello:latest
Successfully copied Flight 'mellow-order' to new Flight 'waiting-leaf' with ID '2d331f0c'

$ seaplane flight list
LOCAL ID  NAME          IMAGE                                                                                                   MIN  MAX  ARCH  API PERMS
d4e877b7  mellow-order  seaplane_demo/nginxdemos/hello@sha256:33d30466bb608f607a8d708d39bf13ec7a908dde1a8a8b228f7f3f4c6a4d1bdf  1    INF  auto  false
2d331f0c  waiting-leaf  seaplane_demo/nginxdemos/hello:latest                                                                   1    INF  auto  false
```

There are a bunch of other things you can do with your Flights as well, see the
`seaplane flight --help` for details.

#### Working with Local Formation Definitions

Remember that we said a Formation is made up of Flights? *Technically* a
Formation is made up of zero or more Formation Configurations. These
configurations define what Flights are utilized, and how they're allowed to
communicate/scale. A Formation can have *zero or more* because having multiple
Configurations will allow you load balance between them! This empowers things
like blue/green deployments, atomic upgrades, all kinds of nifty tools!

But for now, we're staying simple. We're just working with a single
Configuration, which only contains a single Flight. So no load balancing, or
other complexities. Those will come in future chapters!

The reason we bring up this distinction now is if you you're looking for
something similar to `seaplane flight list`, but for Formations; you'll find
it...but it may not make sense, at least without knowing about Formation
Configurations.

So lets check that command now:

```console
$ seaplane formation list
LOCAL ID  NAME         LOCAL  DEPLOYED (GROUNDED)   DEPLOYED (IN AIR)   TOTAL CONFIGURATIONS
8bfe5304  nimble-bike  1      0                     1                   1
```

Notice how it says we have one `LOCAL`, and one `DEPLOYED (IN AIR)`. What do
those mean? The `seaplane formation list` is telling you how many Formation
Configurations it knows about, and what their status is. So it's saying that we
have one local definition (which we created earlier), and one `DEPLOYED (IN
AIR)` which means, "Uploaded to the Seaplane Cloud (Deployed) and set to active
(In Air)."

Let's create another Formation, without any configuration so you can better see
the distinction.

```console
$ seaplane formation create
Successfully created Formation 'kind-week' with ID '86bd3a0c'

$ seaplane formation list
LOCAL ID  NAME         LOCAL  DEPLOYED (GROUNDED)   DEPLOYED (IN AIR)   TOTAL CONFIGURATIONS
8bfe5304  nimble-bike  1      0                     1                   1
86bd3a0c  kind-week    0      0                     0                   0
```

Now we have a new Formation! A perfectly useless Formation, but hey!

OK, let's actually delete that empty Formation and create one a little more
useful. This time when we create the Formation we *will not* be automatically
deploying it to the Seaplane Cloud, so you can see how to do that manually.

```console
$ seaplane formation delete kind-week
Deleted Formation 86bd3a0c72cfc6e6ea2c4c7c37766361d801ea84bab72ae42d4b0d86afd42217

Successfully removed 1 item

$ seaplane formation create --flight mellow-order
Successfully created Formation 'festive-winter' with ID 'd3aa195d'
Successfully created Formation Configuration with ID 'a43c1813'

$ seaplane formation list
LOCAL ID  NAME            LOCAL  DEPLOYED (GROUNDED)   DEPLOYED (IN AIR)   TOTAL CONFIGURATIONS
8bfe5304  nimble-bike     1      0                     1                   1
d3aa195d  festive-winter  1      0                     0                   1
```

Now, notice that `festive-winter` has one local configuration, but none
currently deployed. Let's fix that, by deploying this formation but without
setting it to active.

```console
$ seaplane formation launch --grounded festive-winter
Successfully launched Formation 'festive-winter'
```

The `--grounded` flag tells Seaplane not set the configuration to active, so no
actual traffic can reach it.

We can see this by looking at the Formations again:

```console
$ seaplane formation list
LOCAL ID  NAME            LOCAL  DEPLOYED (GROUNDED)   DEPLOYED (IN AIR)   TOTAL CONFIGURATIONS
8bfe5304  nimble-bike     1      0                     1                   1
d3aa195d  festive-winter  1      1                     0                   1
```

But let's say we *do* want to start up that configuration and make it active.
We can actually just re-pass the same command but without the `--grounded`
flag. Which will make all configurations for a given Formation active (And we
only have one right now.)

```console
$ seaplane formation launch --grounded festive-winter
Successfully launched Formation 'festive-winter'

$ seaplane formation list
LOCAL ID  NAME            LOCAL  DEPLOYED (GROUNDED)   DEPLOYED (IN AIR)   TOTAL CONFIGURATIONS
8bfe5304  nimble-bike     1      0                     1                   1
d3aa195d  festive-winter  1      0                     1                   1
```

Notice `festive-winter` has changed it's configuration status from `GROUNDED`
to `IN AIR`!

### See a Hello World Page

If you've been following along the whole way, you currently have two Formations
running. If you jumped from the [Your First Workload](#your-first-workload)
you'll only have one.

So we'll test out the first one, and leave the second as an experiment for the
reader.

If all went according to plan, you should have a container image running and
addressable from:

```
$ curl https://nimble-bike.seaplane_demo.seaplanet.io/ | head -n 4
<!DOCTYPE html>
<html>
<head>
<title>Hello World</title>
```

Yay!

[//]: # (Links)

[Flightdeck]: https://flightdeck.seaplanet.io/
[Github Releases]: https://github.com/seaplane-io/seaplane/releases
[TOML]: https://toml.io
[doc/CONFIGURATION_SPEC.md]: https://github.com/seaplane-io/seaplane/blob/main/doc/CONFIGURATION_SPEC.md
[SDK]: https://github.com/seaplane-io/seaplane/tree/main/seaplane
[`nginxdemos/hello`]: https://hub.docker.com/r/nginxdemos/hello/
