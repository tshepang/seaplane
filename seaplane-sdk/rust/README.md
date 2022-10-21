# Seaplane Rust SDK

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Dependency Status][deps-image]][deps-link]

The Seaplane Rust SDK allows one to interact with our hosted services using the
Rust language. In fact, it's what our Seaplane CLI tool uses internally!

<!-- vim-markdown-toc GFM -->

* [Prerequisites](#prerequisites)
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

## License

This crate is licensed under the terms of the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/seaplane.svg
[crate-link]: https://crates.io/crates/seaplane
[deps-image]: https://deps.rs/crate/seaplane/status.svg
[deps-link]: https://deps.rs/crate/seaplane
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg

[//]: # (Links)

[flightdeck]: https://flightdeck.cplane.cloud/
[TOML]: https://toml.io
[rustup]: https://rustup.rs
[docs/CONFIGURATION_SPEC.md]: https://github.com/seaplane-io/seaplane/blob/main/docs/CONFIGURATION_SPEC.md
[SDK]: https://github.com/seaplane-io/seaplane/tree/main/seaplane
[`nginxdemos/hello`]: https://hub.docker.com/r/nginxdemos/hello/
[releases]: https://github.com/seaplane-io/seaplane/releases
