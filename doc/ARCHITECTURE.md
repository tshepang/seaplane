# Seaplane CLI and SDK Architecture

This guide covers design decisions and how the pieces fit together.

## Support Matrix

The Seaplane CLI and SDK library are supported on the following Operating Systems and
Architectures:

|         | Linux | Windows | macOS |
| x86_64  | Y     | Y       | Y     |
| aarch64 | Y     | N       | Y     |

Using the following target triples:

- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-gnu`
- `x86_64-apple-darwin`
- `aarch64-unknown-linux-gnu`
- `aarch64-apple-darwin`

## CLI

### Type Naming Conventions

The following naming conventions help simplify complex CLIs.

- All structs representing "raw" CLI flags, options, arguments, and values will use the suffix
`..Args`
- All subcommand *possibilities* will be represented as an enum with the suffix `..Cmds`
  - The variants payloads of these enums will be structs representing the actual CLI arguments,
  flags, and options of their respective subcommands; thus they will naturally use the `..Args`
  suffix
- Subcommands taking nested subcommands will have a single struct field named `cmd` which will be of
the type of an enum (ending with suffix `..Cmds` as discussed) which represents all the possible
subcommands
  - All subcommand enums and structs will be prefixed with full path of their parent. i.e. image the
  subcommand `seaplane account login` which we know will use the suffix `..Args`, but using the
  parent path prefix, the full name would be `SeaplaneAccountLoginArgs`. Likewise, the enum
  representing the possible subcommands of `seaplane account` will use the suffix `..Cmds`, and
  using the parent path prefix, it's full name will be `SeaplaneAccountCmds`.
- Subcommands will use the singular form of a noun, *unless* the operation is operating on multiple
entities. i.e. the subcommand should be `seaplane account` and not `seaplane accounts` as it is only
operating on a single "`account`." The exception is when the operation *is* operating on multiple
items such as a fictional subcommand of `seaplane machines` which performs some action on a grouping
of `machine` objects.

### Configuration

See [CONFIGURATION_SPEC.md](CONFIGURATION_SPEC.md) in this repository for details on the
configuration file format and structure.

There are two primary types used throughout the application "Contexts" and "Configs." These can be
thought of as the "normalized/processed" and "raw" options respectively.

Types named `Config` or that end in `...Cfg` are responsible for (de)serializing raw files.
These files are primarily designed with the *user* and ergonomics as the primary concern. This
means at times, there will be conflicting options, or items which don't fit neatly into a struct
that would optimal to pass around throughout the application. If we did, at each *point of use* the
code would need to ensure all invariants are upheld manually.

Instead, we take the approach that the Context (`Ctx` or types ending in `...Ctx`) are optimal for
passing around the application as all invariants are already check, mutually exclusive options
handled, etc.

#### Example (Color Handling in the CLI)

A good example of this is with the CLI and color handling. First, some back story to understand the
problem at hand.

When we write output we can add color to it by include special ANSI color codes. In a terminal these
get displayed as fancy colors. However, if the output stream is not a terminal, it looks like
garbage printed to the screen/file (i.e. Instead of a red `foo` you'd see `\u001b[31mfoo`). For this
reason, some users only want to color the output in some circumstances with the option to turn on or
off these ANSI color codes.

We have chosen to turn color *on* by default, but allow users to turn it off with a CLI flag
(`--no-color`).

Now imagine a user doesn't agree with our choice of opt-out. So they alias `seaplane` to `seaplane
--no-color`. But then comes a day where that same user *does* want color output, but perhaps only
for a certain invocation? So we also provide a `--color` flag. This may seem silly since our
default is to already color the output. But by adding both a `--no-color` and `--color` flag, and
allowing these two mutually exclusive flags to override each-other our user can now just happily
add `--color` to their command without having to mess about with their alias, or such fuss!

Now if we zoom in at the implementation level, that means we'd have two fields (the real
implementation is slightly different):

```rust
Config {
  color: bool,
  no_color: bool
}
```

But passing around a `Config` struct to any function that needs to output *potentially* colored
output is a problem. Because they now need to check *two* fields and decide what the user *really*
wanted (which is this case is actually harder because it *also* requires checking environments ARGV
and seeing which option came *last*). So instead, we pass around a `Ctx` struct which looks like
this:

```rust
Ctx {
  color: bool
}
```

Where some other initialization and conversion function is responsible for taking in the raw
unprocessed `Config` struct and creating or updating the `Ctx` struct that is optimal to pass
around the application.

Now those use cases that want to check if they should color output or not can look at a single
field and know that all the behind the scenes invariant checking is already done.

We take this same approach with both CLI options and Config Files. Everything is combined,
normalized checked into a `Ctx` struct. Only in very rare (read super early in the application
startup) should a function be looking at the raw `Config` values to make any decision.
