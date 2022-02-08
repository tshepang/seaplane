# CLI

## Type Naming Conventions

The following naming conventions help simplify complex CLIs.

- All structs representing "raw" CLI flags, options, arguments, and values will use the suffix `..Args`
- All subcommand *possibilities* will be represented as an enum with the suffix `..Cmds`
  - The variants payloads of these enums will be structs representing the actual CLI arguments, flags,
    and options of their respective subcommands; thus they will naturally use the `..Args` suffix
- Subcommands taking nested subcommands will have a single struct field named `cmd` which will be
  of the type of an enum (ending with suffix `..Cmds` as discussed) which represents all the
  possible subcommands
  - All subcommand enums and structs will be prefixed with full path of their parent. i.e. image
    the subcommand `seaplane account login` which we know will use the suffix `..Args`, but using
    the parent path prefix, the full name would be `SeaplaneAccountLoginArgs`. Likewise, the enum
    representing the possible subcommands of `seaplane account` will use the suffix `..Cmds`, and
    using the parent path prefix, it's full name will be `SeaplaneAccountCmds`.
- Subcommands will use the singular form of a noun, *unless* the operation is operating on multiple
  entities. i.e. the subcommand should be `seaplane account` and not `seaplane accounts` as it is
  only operating on a single "`account`." The exception is when the operation *is* operating on
  multiple items such as a fictional subcommand of `seaplane machines` which performs some action
  on a grouping of `machine` objects.

## Configuration

See [CONFIGURATION_SPEC.md](CONFIGURATION_SPEC.md) in this repository.
